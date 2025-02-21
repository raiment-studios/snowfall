import { RNG } from './raiment-core/index.ts';
import { EventEmitter } from './raiment-core/index.ts';
import { WorldMap } from './engine/world_map.ts';
import { buildDeck } from './engine/build_deck.ts';

export type JournalDrawRegion = {
    type: 'draw_region';
    card: RegionCard;
    instance: RegionInstance;
    bitmap: string;
};

export type JournalEntry =
    | {
          type: 'markdown';
          content: string;
      }
    | JournalDrawRegion;

export type PlayParams = {
    offsetX?: number;
    offsetY?: number;
    angle?: number;
};

export type Action =
    | {
          type: 'play_card';
          selector: CardSelector;
          params?: PlayParams;
      }
    | {
          type: 'set_position';
          selector: string;
      };

export class Player {
    // PROTOTYPE NOTE: position is given in sector coordinates.
    // Eventually this should be finer grain.
    position = {
        x: 0,
        y: 0,
    };
}

export class World {
    events: EventEmitter = new EventEmitter();

    worldMap = new WorldMap();
    journal = new Array<JournalEntry>();
    player = new Player();

    _rng: RNG;
    _deckRegions = buildDeck();
    _actions = new Array<Action>();

    constructor(seed: number) {
        this._rng = new RNG(seed);

        this.journal.push({
            type: 'markdown',
            content: `
**Welcome to the world of Galthea!**

This is an enormous world plagued by the mysterious force known as 
the Maelstrom that has been ripping apart the fabric of reality.

The first step is to play a "start card". Click [here](action:play_card type:region tag:start_card) to do that!
`,
        });
    }

    enqueue(action: Action) {
        this._actions.push(action);
        this.events.fire('modified');
    }

    _runningActions = false;

    async runActions() {
        // Serialize the async actions into a linear queue so that
        // cards get processed in order (i.e. don't have multiple instances of
        // runActions running at the same time).
        if (this._runningActions) {
            return;
        }

        while (this._actions.length > 0) {
            this._runningActions = true;
            const action: Action = this._actions.shift()!;
            switch (action.type) {
                case 'play_card':
                    {
                        const { selector, params } = action;
                        await this.playCard(selector, params);
                    }
                    break;
                case 'set_position':
                    {
                        const { selector } = action;

                        const pos = this.worldMap.findRandomRegionPosition(this._rng, selector);
                        this.player.position.x = pos[0];
                        this.player.position.y = pos[1];

                        this.journal.push({
                            type: 'markdown',
                            content: `You have been placed at ${selector} (${this.player.position.x}, ${this.player.position.y}).`,
                        });

                        this.events.fire('modified');
                    }
                    break;

                default: {
                    const a = action as any;
                    console.error(`Unknown action type: ${a.type}`);
                    debugger;
                }
            }
            this._runningActions = false;
        }
    }

    async playCard(selector: CardSelector, params: PlayParams = {}) {
        const rng = this._rng;
        const card = this._deckRegions.draw(this._rng, selector);
        const seed = rng.d8192();

        if (card.type === undefined) {
            console.error('Invalid card:', card);
            throw new Error('Card type is undefined');
        }

        switch (card.type) {
            case 'region':
                {
                    const instance = await card.generator(seed, card);
                    instance.seed = seed;

                    // Eventually the angle should be determined by the neighbors defined by the
                    // instance or card
                    const angle = (params.angle ?? this._rng.range(0, 360)) + rng.range(-20, 20);
                    const cx = params.offsetX ?? 0;
                    const cy = params.offsetY ?? Math.round(512 * 0.75);
                    const pos = await this.worldMap.place(instance, cx, cy, angle);

                    this.journal.push({
                        type: 'draw_region',
                        card,
                        instance,
                        bitmap: await this.generateMiniMap(),
                    });

                    // Run the actions on the card
                    for (const action of card.actions.play) {
                        this.enqueue(action);
                    }

                    for (const n of card.neighbors) {
                        const neighbor = this._deckRegions._cards.find((c) => c.id === n.id);
                        if (!neighbor) {
                            continue;
                        }

                        this.enqueue({
                            type: 'play_card',
                            selector: {
                                id: neighbor.id,
                            },
                            params: {
                                angle: n.angle,
                                offsetX: pos[0] + n.offset_x,
                                offsetY: pos[1] - n.offset_y,
                            },
                        });
                    }

                    this.events.fire('modified');
                }
                break;
        }
    }

    async generateMiniMap() {
        function hexToRgb(hex: string): [number, number, number] {
            const match = hex.match(/^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i);
            return match
                ? [
                      parseInt(match[1], 16), //
                      parseInt(match[2], 16),
                      parseInt(match[3], 16),
                  ]
                : [0, 0, 0];
        }

        const rng = RNG.make_random();
        const shades = [1.0, 1.0, 0.98, 0.95, 0.92, 0.9, 0.85];

        // Create a canvas the size of the worldMap
        const canvas = document.createElement('canvas');
        canvas.width = this.worldMap.width;
        canvas.height = this.worldMap.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2d context');
        }

        // Draw the worldMap using the region colors for each pixel
        const map = this.worldMap.map;
        const palette = this.worldMap.palette;
        const width = this.worldMap.width;
        const height = this.worldMap.height;
        const imageData = ctx.createImageData(width, height);
        const data = imageData.data;
        for (let i = 0; i < width * height; i++) {
            const region = palette[map[i]];
            const color = region ? region.color : '#3377FF';
            const rgb = hexToRgb(color);
            const s = rng.select(shades);

            data[i * 4 + 0] = Math.floor(s * rgb[0]);
            data[i * 4 + 1] = Math.floor(s * rgb[1]);
            data[i * 4 + 2] = Math.floor(s * rgb[2]);
            data[i * 4 + 3] = 255;
        }

        // Get the player position and color that pixel yellow
        {
            const x = this.player.position.x + 512;
            const y = this.player.position.y + 512;

            const S = 2;
            for (let dy = -S; dy <= S; dy++) {
                for (let dx = -S; dx <= S; dx++) {
                    const ex = x + dx;
                    const ey = y + dy;
                    if (ex < 0 || ex >= width || ey < 0 || ey >= height) {
                        continue;
                    }

                    const index = ey * width + ex;
                    data[index * 4 + 0] = 255;
                    data[index * 4 + 1] = 255;
                    data[index * 4 + 2] = 0;
                }
            }
        }

        // Copy the image data back to the canvas and return a data URL
        ctx.putImageData(imageData, 0, 0);
        return canvas.toDataURL();
    }
}

// A particular Region Card has a region generator
export type RegionGenerator = (seed: number, card: RegionCard) => Promise<RegionInstance>;

export type RegionInstance = {
    id: string;
    title: string;
    seed: number;
    color: string;
    bitmap: string;
};

export type RegionCard = {
    type: 'region';
    id: string;
    tags: string[];
    title: string;
    description: string;
    rarity: number; // 1-1000
    color: string;
    size: number;
    image: string;
    neighbors: Array<{
        id: string;
        angle: number;
        offset_x: number;
        offset_y: number;
    }>;
    generator: RegionGenerator;
    actions: {
        play: Array<Action>;
    };
};

export type CardSelector = {
    id?: string;
    type?: string;
    tag?: string;
};

export class Deck {
    _cards: RegionCard[] = [];

    add(...partials: Array<RegionCard>) {
        this._cards.push(...partials);
    }

    select(rng: RNG): RegionCard {
        const i = rng.selectIndexWeighted(this._cards, (c) => c.rarity);
        return this._cards[i];
    }

    draw(rng: RNG, selector: CardSelector = {}): RegionCard {
        let cards = this._cards;
        if (selector.id) {
            cards = cards.filter((c) => c.id === selector.id);
        }
        if (selector.type) {
            cards = cards.filter((c) => c.type === selector.type);
        }
        if (selector.tag !== undefined) {
            const tag: string = selector.tag; // <-- keeps TypeScript type checking happy
            cards = cards.filter((c) => c.tags.includes(tag));
        }

        const card = rng.pluckWeighted(cards, 'rarity');
        this._cards = this._cards.filter((c) => c !== card);
        return card;
    }
}
