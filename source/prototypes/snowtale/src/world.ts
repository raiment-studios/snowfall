import { RNG } from './raiment-core/index.ts';
import { EventEmitter } from './raiment-core/index.ts';
import chroma from 'chroma-js';
import { ImageMutator } from './views/image_mutator.tsx';

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

/**
 * For prototyping purposes, let's fix the map to 1024x1024 sectors.  We can
 * create dynamically sized map later.
 */
export class WorldMap {
    palette: (RegionInstance | null)[] = [];
    map: Uint16Array = new Uint16Array(1024 * 1024);

    constructor() {
        this.palette.push(null);
        for (let i = 0; i < 1024 * 1024; i++) {
            this.map[i] = 0;
        }
    }

    async place(region: RegionInstance, x: number, y: number, deg: number) {
        let cx = x + 512;
        let cy = y + 512;

        const image = new Image();
        image.src = region.bitmap;
        await new Promise((resolve) => {
            image.onload = resolve;
        });

        cx -= Math.floor(image.width / 2);
        cy -= Math.floor(image.height / 2);

        const index = this.palette.length;
        this.palette.push(region);

        // Get Image Data for image
        const canvas = document.createElement('canvas');
        canvas.width = image.width;
        canvas.height = image.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2d context');
        }
        ctx.drawImage(image, 0, 0);
        const imageData = ctx.getImageData(0, 0, image.width, image.height);
        const data = imageData.data;

        let placement = [0, 0];

        const original = this.map.slice();
        let dist = 0;
        let attempts = 100;
        do {
            const ox = Math.floor(dist * Math.cos((-deg * Math.PI) / 180));
            const oy = Math.floor(dist * Math.sin((-deg * Math.PI) / 180));

            let total = 0;
            let skipped = 0;
            for (let ix = 0; ix < image.width; ix++) {
                for (let iy = 0; iy < image.height; iy++) {
                    const alpha = data[(ix + iy * image.width) * 4 + 3];
                    if (alpha === 0) {
                        continue;
                    }
                    total += 1;

                    const px = ox + cx + ix;
                    const py = oy + cy + iy;
                    const p = px + py * 1024;
                    if (this.map[p] !== 0) {
                        skipped += 1;
                        continue;
                    }
                    this.map[p] = index;
                }
            }

            if (skipped > total * 0.1) {
                dist += 6;
                this.map = original.slice();
            } else {
                placement = [ox, oy];
                break;
            }
            attempts -= 1;
        } while (attempts > 0);

        return placement;
    }

    toDataURL(): string {
        const rng = RNG.make_random();
        const shades = [1.0, 1.0, 0.98, 0.95, 0.92, 0.9];

        const canvas = document.createElement('canvas');
        canvas.width = 1024;
        canvas.height = 1024;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2d context');
        }

        const imageData = ctx.getImageData(0, 0, 1024, 1024);
        const data = imageData.data;

        for (let i = 0; i < 1024 * 1024; i++) {
            const region = this.palette[this.map[i]];
            if (region) {
                const rgb = hexToRgb(region.color);
                const s = rng.select(shades);
                const index = i * 4;
                data[index + 0] = Math.floor(s * rgb[0]);
                data[index + 1] = Math.floor(s * rgb[1]);
                data[index + 2] = Math.floor(s * rgb[2]);
                data[index + 3] = 255;
            } else {
                const index = i * 4;
                data[index + 0] = 0;
                data[index + 1] = 64;
                data[index + 2] = 128;
                data[index + 3] = 255;
            }
        }
        ctx.putImageData(imageData, 0, 0);
        return canvas.toDataURL();
    }
}

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

    async runActions() {
        if (this._actions.length === 0) {
            return;
        }
        const actions = this._actions.slice();
        this._actions = [];

        while (actions.length > 0) {
            const action = actions.shift()!;
            switch (action.type) {
                case 'play_card': {
                    const { selector, params } = action;
                    await this.playCard(selector, params);
                    break;
                }
            }
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
                    const cy = params.offsetY ?? 0;
                    const pos = await this.worldMap.place(instance, cx, cy, angle);

                    this.journal.push({
                        type: 'draw_region',
                        card,
                        instance,
                        bitmap: this.worldMap.toDataURL(),
                    });

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
        generator: async (seed: number, card: RegionCard) => {
            throw new Error('Generator not implemented');
        },
    };

    if (partial.id === undefined && partial.title) {
        partial.id = partial.title.toLowerCase().replace(/\s+/g, '-');
    }
    if (partial.description !== undefined) {
        partial.description = partial.description.trim();
    }

    const merged = { ...template, ...partial };
    return merged;
}

function buildDeck(): Deck {
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
                        offset_x: 60,
                        offset_y: 30,
                    },
                    {
                        id: 'crags',
                        angle: 165,
                        offset_x: -30,
                        offset_y: 30,
                    },
                    {
                        id: 'midland',
                        angle: 90,
                        offset_x: 60,
                        offset_y: 30,
                    },
                ],
                generator: async (seed: number, card: RegionCard) => {
                    return generateInstance(seed, card, 'Haven');
                },
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
                generator: async (seed: number, card: RegionCard) => {
                    return generateInstance(seed, card, 'Redrock');
                },
            },
            {
                id: 'midland',
                title: 'Midland',
                rarity: 500,
                description: '',
                color: '#3C3',
                size: 220,
                image: '/static/region-bitmap-midland.png',
                generator: async (seed: number, card: RegionCard) => {
                    return generateInstance(seed, card, 'Midland');
                },
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
                        offset_x: -60,
                        offset_y: 60,
                    },
                ],
                generator: async (seed: number, card: RegionCard) => {
                    return generateInstance(seed, card, "Wizard's Crags");
                },
            },
            {
                id: 'cores',
                title: "Core's Coast",
                rarity: 1000,
                description: '',
                color: '#275',
                size: 90,
                image: '/static/region-bitmap-cores.png',
                generator: async (seed: number, card: RegionCard) => {
                    return generateInstance(seed, card, "Core's Coast");
                },
            },
        ].map(normalize)
    );

    return deck;
}
