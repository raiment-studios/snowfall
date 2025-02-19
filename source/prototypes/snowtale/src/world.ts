import { RNG } from './raiment-core/index.ts';
import { EventEmitter } from './raiment-core/index.ts';
import chroma from 'chroma-js';
import { ImageMutator } from './views/image_mutator.tsx';
import { WorldMap } from './engine/world_map.ts';

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

export type Action = {
    type: 'play_card';
    selector: CardSelector;
    params?: PlayParams;
};

export class World {
    events: EventEmitter = new EventEmitter();

    worldMap = new WorldMap();
    journal = new Array<JournalEntry>();

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
            const action = this._actions.shift()!;
            switch (action.type) {
                case 'play_card': {
                    const { selector, params } = action;
                    console.log('Playing card', selector);
                    await this.playCard(selector, params);
                    break;
                }
            }
            this._runningActions = false;
        }
    }

    async playCard(selector: CardSelector, params: PlayParams = {}) {
        const rng = this._rng;
        const card = this._deckRegions.draw(this._rng, selector);
        const seed = rng.d8192();

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
                        bitmap: this.worldMap.toDataURL(),
                    });
                    console.log('Placed region', card.title, pos);

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

    async drawRegion() {
        const card = this._deckRegions.draw(this._rng);
        const seed = this._rng.d8192();
        const instance = await card.generator(seed, card);
        instance.seed = seed;

        // Eventually the angle should be determined by the neighbors defined by the
        // instance or card
        const angle = this._rng.range(0, 360);
        await this.worldMap.place(instance, 0, 0, angle);

        this.journal.push({
            type: 'draw_region',
            card,
            instance,
            bitmap: this.worldMap.toDataURL(),
        });
        this.events.fire('modified');
    }
}

// A particular Region Card has a region generator
export type RegionGenerator = (seed: number, card: RegionCard) => Promise<RegionInstance>;

export type RegionInstance = {
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
};

export type CardSelector = {
    id?: string;
    type?: string;
    tag?: string;
};

class Deck {
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

const generateInstance = async (
    seed: number,
    card: RegionCard,
    title: string
): Promise<RegionInstance> => {
    const rng = new RNG(seed);
    let { color } = card;

    // Jitter the hue of the color a little
    const hsl = chroma(color).hsl();
    hsl[0] += rng.range(-10, 10);
    color = chroma.hsl(...hsl).hex();

    const dim = Math.ceil(card.size * rng.range(0.8, 1.2));

    const deg = rng.range(-30, 30);
    const bitmap = await new ImageMutator(card.image)
        .rotate(deg)
        .colorize(color)
        .autocrop()
        .resize(dim, dim)
        .blur(3)
        .colorize(color)
        .clampAlpha()
        .speckleColor()
        .toDataURL();

    return {
        title,
        seed,
        color,
        bitmap,
    };
};

let _globalCounter = 0;

function normalize(partial: Partial<RegionCard>): RegionCard {
    _globalCounter += 1;
    const template: RegionCard = {
        type: 'region',
        id: `unknown-${_globalCounter}`,
        tags: [],
        title: 'Untitled',
        description: '',
        rarity: 1000,
        color: '#FF00FF',
        size: 100,
        image: '',
        neighbors: [],
        generator: (seed: number, card: RegionCard) => {
            throw new Error('Generator not implemented');
        },
    };

    if (partial.id === undefined && partial.title) {
        partial.id = partial.title.toLowerCase().replace(/\s+/g, '-');
    }
    if (partial.description !== undefined) {
        partial.description = partial.description.trim();
    }
    if (partial.generator === undefined) {
        partial.generator = (seed: number, card: RegionCard) => {
            return generateInstance(seed, card, partial.title ?? template.title);
        };
    }

    const merged = { ...template, ...partial };

    merged.size = Math.round(merged.size * 1.75);

    return merged;
}

function buildDeck(): Deck {
    const deck = new Deck();
    deck.add(
        ...[
            {
                id: 'haven',
                title: 'Haven',
                rarity: 1000,
                tags: ['start_card'],
                description: `
The starting point of the game. Lined with small harbor towns to the southwest.
Wayland artifacts are prevalent here, reducing the impact of the Maelstrom.
            `,
                color: '#25b585',
                size: 70,
                image: '/static/region-bitmap-haven.png',
                neighbors: [
                    {
                        id: 'redrock',
                        angle: 0,
                        offset_x: 40,
                        offset_y: 20,
                    },
                    {
                        id: 'crags',
                        angle: 165,
                        offset_x: -10,
                        offset_y: 10,
                    },
                    {
                        id: 'midland',
                        angle: 90,
                        offset_x: 30,
                        offset_y: 10,
                    },
                ],
            },
            {
                id: 'redrock',
                title: 'Redrock',
                rarity: 500,
                description: `
A sparsely populated region of sand and coarse dirt. Vegetation is limited here 
due to the raw terrain.
                        `,
                color: '#ae8030',
                size: 160,
                image: '/static/region-bitmap-redrock.png',
            },
            {
                id: 'midland',
                title: 'Midland',
                rarity: 500,
                description: '',
                color: '#2b1',
                size: 200,
                image: '/static/region-bitmap-midland.png',
                neighbors: [
                    {
                        id: 'brook-hills',
                        angle: 80,
                        offset_x: -20,
                        offset_y: 10,
                    },
                ],
            },
            {
                id: 'crags',
                title: "Wizard's Crags",
                rarity: 1000,
                description: '',
                color: '#666',
                size: 70,
                image: '/static/region-bitmap-crags.png',
                neighbors: [
                    {
                        id: 'cores',
                        angle: 110,
                        offset_x: -30,
                        offset_y: 30,
                    },
                ],
            },
            {
                id: 'cores',
                title: "Core's Coast",
                rarity: 1000,
                description: '',
                color: '#275',
                size: 90,
                image: '/static/region-bitmap-cores.png',
            },
            {
                id: 'brook-hills',
                title: 'Brook Hills',
                rarity: 1000,
                description: '',
                color: '#7a1',
                size: 170,
                image: '/static/region-bitmap-brook-hills.png',
                neighbors: [
                    {
                        id: 'highwall',
                        angle: 105,
                        offset_x: 0,
                        offset_y: 10,
                    },
                ],
            },
            {
                id: 'highwall',
                title: 'Highwall',
                rarity: 1000,
                description: '',
                color: '#952',
                size: 130,
                image: '/static/region-bitmap-highwall.png',
                neighbors: [
                    {
                        id: 'forest-stairs',
                        angle: 103,
                        offset_x: 5,
                        offset_y: 1,
                    },
                    {
                        id: 'barrens',
                        angle: 140,
                        offset_x: -30,
                        offset_y: 1,
                    },
                    {
                        id: 'boundary',
                        angle: 90,
                        offset_x: 0,
                        offset_y: 2,
                    },
                    {
                        id: 'far-north',
                        angle: 90,
                        offset_x: 0,
                        offset_y: 3,
                    },
                ],
            },
            {
                id: 'forest-stairs',
                title: 'Forest Stairs',
                rarity: 1000,
                description: '',
                color: '#250',
                size: 150,
                image: '/static/region-bitmap-forest-stairs.png',
            },
            {
                id: 'barrens',
                title: 'Barrens',
                rarity: 1000,
                description: '',
                color: '#534',
                size: 120,
                image: '/static/region-bitmap-barrens.png',
            },
            {
                id: 'boundary',
                title: 'Boundary',
                rarity: 1000,
                description: '',
                color: '#676',
                size: 120,
                image: '/static/region-bitmap-boundary.png',
            },
            {
                id: 'far-north',
                title: 'Far North',
                rarity: 1000,
                description: '',
                color: '#aac',
                size: 220,
                image: '/static/region-bitmap-far-north.png',
            },
        ].map(normalize)
    );

    return deck;
}
