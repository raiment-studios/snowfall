import { RegionCard, RegionInstance, Deck } from '../world.ts';
import chroma from 'chroma-js';
import { RNG } from '../raiment-core/index.ts';
import { ImageMutator } from '../views/image_mutator.tsx';

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
        id: card.id,
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
        actions: {
            play: [],
        },
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
    Object.assign(merged.actions, partial.actions);

    merged.size = Math.round(merged.size * 1.75);

    return merged;
}

export function buildDeck(): Deck {
    const partials: Array<Partial<RegionCard>> = [
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
            actions: {
                play: [{ type: 'set_position', selector: 'haven' }],
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
    ];

    const deck = new Deck();
    deck.add(...partials.map(normalize));

    return deck;
}
