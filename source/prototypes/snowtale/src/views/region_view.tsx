import React, { JSX } from 'react';
import { RNG } from '../raiment-core/index.ts';
import { Div, css } from '../raiment-ui/index.ts';
import { ImageMutator } from './image_mutator.tsx';
import chroma from 'chroma-js';
import { RegionCard, RegionInstance, World, JournalDrawRegion } from '../world.ts';

class Deck {
    _cards: RegionCard[] = [];

    add(...partials: Array<RegionCard>) {
        this._cards.push(...partials);
    }

    select(rng: RNG): RegionCard {
        const i = rng.selectIndexWeighted(this._cards, (c) => c.rarity);
        return this._cards[i];
    }

    draw(rng: RNG): RegionCard {
        return rng.pluckWeighted(this._cards, 'rarity');
    }
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

export function DrawRegionView({
    world,
    entry,
}: {
    world: World;
    entry: JournalDrawRegion;
}): JSX.Element {
    const { card, instance, bitmap } = entry;

    return (
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
                <Div>{instance.seed}</Div>
            </Div>
            <Div
                css={css`
                    .self {
                        width: 480px;

                        .title {
                            font-size: 120%;
                            font-weight: bold;
                        }
                    }
                `}
            >
                <Div cl="title">{instance.title}</Div>
                <Div
                    css={css`
                        .self {
                            width: 100%;
                            height: 8px;
                            background-color: ${instance.color};
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
                    <img style={{ margin: 8 }} src={instance.bitmap} />
                </Div>
                <Div>
                    TODO: add to global map, add political factions (60/35/5), colors for each,
                    maelstrom factor, and other properties.
                </Div>
            </Div>
            <Div>
                <h3>World Map</h3>
                <Div
                    css={css`
                        .self {
                            box-sizing: content-box;
                            width: 512px;
                            height: 512px;
                            border: solid 1px #000;
                        }
                    `}
                >
                    <img src={bitmap} width={512} height={512} />
                </Div>
            </Div>
        </Div>
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
                .toDataURL();
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

                            .title {
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
                            margin: 8px 4px;
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
                        flex-basis: 24px;
                        border-radius: 6px;

                        .id {
                            font-size: 50%;
                            color: #0008;
                            font-style: italic;
                            line-height: 90%;
                            padding-left: 4px;
                        }
                    }
                }
            `}
        >
            <Div cl="header">
                <Div cl="left">
                    <Div cl="title">{card.title}</Div>
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
                            src={image ?? undefined}
                        />
                    </Div>
                    <Div cl="image-right"></Div>
                </Div>
                <Div cl="description serif">{card.description}</Div>
            </Div>
            <Div cl="footer">
                <Div cl="id">{card.id}</Div>
            </Div>
        </Div>
    );
}
