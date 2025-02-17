/**
 
    RegionCard
        A deterministic RegionInst generator

    RegionInst
        Wrapper class on RegionInstData to make it easier to work with

    RegionInstData
        The plain old data of the region instance

    RegionView
        A React view a RegionInst

 */

import React, { JSX } from 'react';
import { RNG } from '../raiment-core/index.ts';
import { Div, css } from '../raiment-ui/index.ts';
import { ImageMutator } from './image_mutator.tsx';
import chroma from 'chroma-js';

// A particular Region Card has a region generator
type RegionGenerator = (seed: number, card: RegionCard) => Promise<RegionInstData>;

type RegionInstData = {
    name: string;
    seed: number;
    color: string;
    bitmap: string;
};

type RegionCard = {
    type: 'region';
    name: string;
    description: string;
    color: string;
    image: string;
    props: { [key: string]: any };
    generator: RegionGenerator;
};

class Deck {
    _cards: RegionCard[] = [];

    add(...partials: Array<Partial<RegionCard>>) {
        for (const input of partials) {
            const partial = { ...input };

            if (partial.description !== undefined) {
                partial.description = partial.description.trim();
            }

            const template: RegionCard = {
                type: 'region',
                name: 'Untitled',
                description: '',
                color: '#FF00FF',
                image: '',
                props: {},
                generator: async (seed: number, card: RegionCard) => {
                    throw new Error('Generator not implemented');
                },
            };
            const card: RegionCard = { ...template, ...partial };
            this._cards.push(card);
        }
    }

    select(rng: RNG): RegionCard {
        return rng.select(this._cards);
    }
}

export function hexToRgb(hex: string): [number, number, number] {
    const match = hex.match(/^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i);
    return match
        ? [
              parseInt(match[1], 16), //
              parseInt(match[2], 16),
              parseInt(match[3], 16),
          ]
        : [0, 0, 0];
}

function buildDeck(): Deck {
    const deck = new Deck();

    const mutateImage = (url: string, deg: number, color: string): Promise<string> => {
        return new ImageMutator(url)
            .rotate(deg)
            .colorize(color)
            .autocrop()
            .resize(256, 256)
            .blur()
            .blur()
            .blur()
            .colorize(color)
            .clampAlpha()
            .speckleColor()
            .run();
    };

    deck.add(
        {
            name: 'Haven',
            description: `
The starting point of the game. Lined with small harbor towns to the southwest.
Wayland artifacts are prevalent here, reducing the impact of the Maelstrom.
            `,
            color: '#25b585',
            image: '/static/region-bitmap-00.png',
            generator: async (seed: number, card: RegionCard) => {
                const rng = new RNG(seed);
                const { color } = card;

                const bitmap = await mutateImage(card.image, rng.range(-45, 45), color);

                return {
                    name: 'Haven',
                    seed,
                    color,
                    bitmap,
                    props: {},
                };
            },
        },
        {
            name: 'Redrock',
            description: `
A sparsely populated region of sand and coarse dirt. Vegetation is limited here 
due to the raw terrain.
                        `,
            color: '#ae8030',
            image: '/static/region-bitmap-01.png',
            generator: async (seed: number, card: RegionCard) => {
                const rng = new RNG(seed);
                const { color } = card;
                const bitmap = await mutateImage(card.image, rng.range(-30, 30), color);
                return {
                    name: 'Redrock',
                    seed,
                    color,
                    bitmap,
                    props: {},
                };
            },
        }
    );

    return deck;
}

function useGoogleFont(url: string) {
    const id = `font-${encodeURIComponent(url)}`;

    React.useEffect(() => {
        const existing = document.getElementById(id);
        if (existing) {
            return;
        }
        const link = document.createElement('link');
        link.id = id;
        link.rel = 'stylesheet';
        link.href = url;
        document.getElementsByTagName('head')[0].appendChild(link);
    }, [url]);
}

export function RegionView({ seed }: { seed: number }): JSX.Element {
    const [{ card, inst }, setInst] = React.useState<{
        card?: RegionCard;
        inst?: RegionInstData;
    }>({});

    React.useEffect(() => {
        const go = async () => {
            const deck = buildDeck();
            const rng = new RNG(seed);

            const card = deck.select(rng);
            const inst = await card.generator(rng.d8192(), card);
            setInst({
                card,
                inst,
            });
        };
        go();
    }, [seed]);

    if (!inst || !card) {
        return <></>;
    }

    return (
        <div>
            <Div
                css={css`
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    gap: 16px;
                `}
            >
                <SmallCard card={card} />
                <Div
                    css={css`
                        .self {
                            font-size: 140%;
                            text-align: center;
                        }
                    `}
                >
                    <Div>generate →</Div>
                    <Div>{inst.seed}</Div>
                </Div>
                <Div
                    css={css`
                        .self {
                            width: 480px;

                            .name {
                                font-size: 120%;
                                font-weight: bold;
                            }
                        }
                    `}
                >
                    <Div cl="name">{inst.name}</Div>
                    <Div
                        css={css`
                            .self {
                                width: 100%;
                                height: 8px;
                                background-color: ${inst.color};
                            }
                        `}
                    />
                    <Div
                        css={css`
                            display: flex;
                            flex-direction: row;
                            align-items: center;
                            justify-content: center;
                        `}
                    >
                        <img style={{ margin: 8 }} src={inst.bitmap} />
                    </Div>
                    <Div>
                        TODO: add to global map, add political factions (60/35/5), colors for each,
                        maelstrom factor, and other properties.
                    </Div>
                </Div>
            </Div>
        </div>
    );
}

function SmallCard({ card }: { card: RegionCard }): JSX.Element {
    const [image, setImage] = React.useState<string | null>(null);

    useGoogleFont(
        'https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&family=Noto+Serif:ital,wght@0,100..900;1,100..900&display=swap'
    );

    React.useEffect(() => {
        const go = async () => {
            const canvas = await new ImageMutator(card.image) //
                .autocrop()
                .resize(320, 320)
                .colorize(card.color)
                .speckleColor()
                .run();
            setImage(canvas);
        };
        go();
    }, [card.image]);

    const backgroundColor1 = chroma(card.color).darken(2.0).hex();
    const backgroundColor2 = chroma(card.color).mix('white', 0.75).hex();

    return (
        <Div
            css={css`
                .self {
                    display: flex;
                    flex-direction: column;
                    box-sizing: border-box;
                    width: 600px;
                    height: 1000px;
                    margin: 4px;
                    border: solid 1px #555;
                    border-radius: 6px;
                    padding: 4px;
                    background-color: ${backgroundColor1};
                    font-family: 'Noto Serif';
                    font-size: 40px;
                    zoom: 0.45;
                    box-shadow: 4px 4px 16px #0003;

                    .sans {
                        font-family: 'Noto Sans', sans-serif;
                    }
                    .serif {
                        font-family: 'Noto Serif', sans-serif;
                    }

                    .header {
                        display: flex;
                        flex-direction: row;
                        justify-content: space-between;
                        padding: 1px 4px;
                        margin-bottom: 6px;
                        background-color: ${backgroundColor2};
                        flex-grow: 0;
                        flex-shrink: 0;
                        border-radius: 6px;

                        .left {
                            display: flex;
                            flex-direction: column;
                            padding: 0 0 4px;

                            .name {
                                font-weight: bold;
                                letter-spacing: 0.3px;
                                line-height: 90%;
                                margin-top: 2px;
                            }
                            .type {
                                font-size: 50%;
                                color: #0008;
                                font-style: italic;
                                line-height: 90%;
                                padding-left: 4px;
                            }
                        }
                        .rarity {
                            display: flex;
                            flex-direction: row;
                            align-items: center;
                            gap: 4px;
                            font-size: 75%;
                            color: #000c;
                            line-height: 100%;
                            font-family: 'Noto Sans';

                            .icon {
                                transform-origin: center;
                                transform: rotate(2deg) translate(0, -2px);
                            }
                            .value {
                                font-size: 80%;
                            }
                        }
                    }
                    .body {
                        display: flex;
                        flex-direction: column;
                        flex-grow: 1;
                        padding: 1px;
                        margin-bottom: 8px;
                        border-radius: 6px;
                        background-color: #fffc;

                        .image-row {
                            display: flex;
                            flex-direction: row;

                            .image-left {
                                flex: 1 0 0;
                                background-color: #0003;
                            }
                            .image {
                                flex-grow: 0;
                                flex-shrink: 0;
                                background-color: #0001;
                                width: 320px;
                                height: 320px;
                                margin: 0 auto;
                                border: solid 1px #0004;
                            }
                            .image-right {
                                flex: 1 0 0;
                                background-color: #0003;
                            }
                        }

                        .description {
                            margin: 8px 12px;
                            padding: 8px;
                            border: solid 1px #0003;
                            border-radius: 4px;
                            font-size: 60%;
                            line-height: 110%;
                            flex-grow: 1;
                            background: #0001;
                        }
                    }

                    .footer {
                        padding: 1px 4px;
                        background-color: #fff8;
                        font-size: 90%;
                        flex-grow: 0;
                        flex-shrink: 0;
                        flex-basis: 48px;
                        border-radius: 6px;
                    }
                }
            `}
        >
            <Div cl="header">
                <Div cl="left">
                    <Div cl="name">{card.name}</Div>
                    <Div cl="type">{card.type}</Div>
                </Div>
                <Div>
                    <Div cl="rarity">
                        <Div cl="icon">⚄</Div>
                        <Div cl="value">1000</Div>
                    </Div>
                </Div>
            </Div>
            <Div cl="body">
                <Div cl="image-row">
                    <Div cl="image-left"></Div>
                    <Div cl="image">
                        <img
                            style={{
                                width: '100%',
                                height: '100%',
                                display: 'block',
                                objectFit: 'contain',
                            }}
                            src={image ?? ''}
                        />
                    </Div>
                    <Div cl="image-right"></Div>
                </Div>
                <Div cl="description serif">{card.description}</Div>
            </Div>
            <Div cl="footer"></Div>
        </Div>
    );
}
