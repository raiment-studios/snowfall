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

// A particular Region Card has a region generator
type RegionGenerator = (seed: number, props: { [key: string]: any }) => Promise<RegionInstData>;

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
                props: {},
                generator: async (seed: number, props: { [key: string]: any }) => {
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

function rotateImage(src: string, deg: number, color: string): Promise<string> {
    const rgb = hexToRgb(color);

    return new Promise((resolve, reject) => {
        const image = new Image();
        image.crossOrigin = 'anonymous';
        image.src = src;

        image.onload = () => {
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');

            if (!ctx) {
                reject(new Error('Canvas context is not supported'));
                return;
            }

            const angle = deg * (Math.PI / 180);
            const sin = Math.abs(Math.sin(angle));
            const cos = Math.abs(Math.cos(angle));

            canvas.width = image.width * cos + image.height * sin;
            canvas.height = image.width * sin + image.height * cos;

            ctx.translate(canvas.width / 2, canvas.height / 2);
            ctx.rotate(angle);
            ctx.drawImage(image, -image.width / 2, -image.height / 2);

            // Get the pixels, change all the white ones to the color,
            // and write it back to the canvas
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const data = imageData.data;
            for (let i = 0; i < data.length; i += 4) {
                if (data[i + 3] !== 0) {
                    data[i] = rgb[0];
                    data[i + 1] = rgb[1];
                    data[i + 2] = rgb[2];
                    data[i + 3] = 255;
                }
            }
            ctx.putImageData(imageData, 0, 0);

            resolve(canvas.toDataURL());
        };

        image.onerror = (err) => reject(err);
    });
}

function buildDeck(): Deck {
    const deck = new Deck();

    deck.add(
        {
            name: 'Haven',
            description: `
The starting point of the game. Lined with small harbor towns to the southwest. Wayland artifacts are prevalent here, reducing the impact of the Maelstrom.
            `,
            generator: async (seed: number, props: { [key: string]: any }) => {
                const rng = new RNG(seed);
                const color = '#25b585';
                const bitmap = await rotateImage(
                    '/static/region-bitmap-00.png',
                    rng.range(-45, 45),
                    color
                );
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
A sparsely populated region of sand and coarse dirt. Vegetation is limited here due to the raw terrain.
                        `,
            generator: async (seed: number, props: { [key: string]: any }) => {
                const rng = new RNG(seed);
                const color = '#ae8030';
                const bitmap = await rotateImage(
                    '/static/region-bitmap-01.png',
                    rng.range(-30, 30),
                    color
                );
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

function useGoogleFont(url) {
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
            const inst = await card.generator(rng.d8192(), card.props);
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
                    <img src={inst.bitmap} />
                    <Div>
                        TODO: add political factions (60/35/5), colors for each, maelstrom factor,
                        and other properties. Make the region images larger.
                    </Div>
                </Div>
            </Div>
        </div>
    );
}

function SmallCard({ card }: { card: RegionCard }): JSX.Element {
    useGoogleFont(
        'https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&family=Noto+Serif:ital,wght@0,100..900;1,100..900&display=swap'
    );

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
                    background-color: #223e78;
                    font-family: 'Noto Serif';
                    font-size: 40px;
                    zoom: 0.45;

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
                        background-color: #fffc;
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
                                line-height: 80%;
                                margin-top: 2px;
                            }
                            .type {
                                font-size: 70%;
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
                        flex-grow: 1;
                        padding: 4px;
                        margin-bottom: 8px;
                        border-radius: 6px;
                        background-color: #fff9;

                        .image {
                            background-color: #0003;
                            border-radius: 2px;
                            width: 320px;
                            height: 320px;
                            margin: 0 auto;
                        }

                        .description {
                            margin: 8px 12px;
                            font-size: 70%;
                            line-height: 110%;
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
                <Div cl="image"></Div>
                <Div cl="description serif">{card.description}</Div>
            </Div>
            <Div cl="footer"></Div>
        </Div>
    );
}
