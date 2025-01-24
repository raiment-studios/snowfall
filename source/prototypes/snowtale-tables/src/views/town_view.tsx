import React, { JSX } from 'react';
import { RNG } from '../snowfall-core/index.ts';
import { Div, Flex, css } from '../snowfall-ui/index.ts';

export function TownView({ seed }: { seed: number }): JSX.Element {
    const inst = React.useMemo(() => generate(seed), [seed]);

    return (
        <Div
            css={css`
                display: flex;
                flex-direction: column;
                box-sizing: border-box;
                width: 300px;
                height: 500px;
                padding: 4px;
                border-radius: 8px;
                background-color: #302431;
                box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.5);

                .topbar {
                    display: flex;
                    justify-content: space-between;
                    padding: 4px;
                    border-bottom: 1px solid #666;
                    background-color: rgba(255, 255, 255, 0.65);
                    border: solid 1px #111;
                    border-radius: 8px;
                    font-weight: bold;
                    padding: 0 4px;
                }
                .body {
                    flex-grow: 1;
                    margin: 4px 0 4px 0;
                    padding: 4px;
                    border-radius: 4px;
                    background-color: rgba(255, 255, 255, 0.7);
                }
                .bottombar {
                    display: flex;
                    justify-content: space-between;
                    padding: 4px;
                    border-bottom: 1px solid #666;
                    background-color: rgba(255, 255, 255, 0.25);
                    border: solid 1px #111;
                    border-radius: 8px;
                    font-size: 80%;
                    font-weight: 100;
                    padding: 0 4px;
                    margin: 0 0 4px 0;
                }
            `}
        >
            <Div cn="topbar">
                <Div cn="name">{inst.name}</Div>
                <Div>{inst.type}</Div>
            </Div>
            <Div cn="body">
                <Table obj={inst} skip="id type name seed props" />
                <Table obj={inst.props} />
            </Div>
            <Div cn="bottombar">
                <Div></Div>
                <Div>
                    {inst.id} {inst.seed}
                </Div>
            </Div>
        </Div>
    );
}

function Table({ obj, skip }: { obj: Record<string, any>; skip?: string }): JSX.Element {
    const skipList = skip ? skip.split(' ').map((s) => s.trim()) : [];

    return (
        <Flex col>
            {Object.entries(obj)
                .filter(([key]) => {
                    return !skipList.includes(key);
                })
                .map(([key, value]) => (
                    <Flex key={key} row>
                        {typeof value !== 'object' ? (
                            <>
                                <Flex w="8em">{key}</Flex>
                                <Flex>{`${value}`}</Flex>
                            </>
                        ) : (
                            <>
                                <Flex w="1em"></Flex>
                                <Flex col>
                                    {Object.entries(value).map(([key, value]) => (
                                        <Flex key={key} row>
                                            <Flex w="8em">{key}</Flex>
                                            <Flex>{`${value}`}</Flex>
                                        </Flex>
                                    ))}
                                </Flex>
                            </>
                        )}
                    </Flex>
                ))}
        </Flex>
    );
}

type Town = {} | any;

function generate(seed: number): Town {
    const rng = new RNG(seed);

    function sel<T>(t: string, s: string): string {
        const arr = s.split(' ').map((s) => s.trim());
        return t + rng.select(arr);
    }

    const nameFn = rng.select([
        () => sel('River', 'wood stone hill dale brook bend'),
        () => sel('Wood', 'ville town river burr'),
        () => 'Bramblewood',
        () => 'Nors',
        () => 'Mills',
        () => 'Bellfound',
        () => 'Sisters',
        () => 'Shellborne',
        () => 'Hilltop',
        () => 'Dale',
        () => 'Bend',
        () => 'Burr',
        () => 'Vale',
        () => 'Fell',
        () => sel('Plain', 'view town glen dale'),
        () => sel('Oak', 'sill tree field'),
        () => 'Rock',
        () => sel('Bill', 'yard meadow'),
        () => sel('Middle', 'view vale town ton'),
        () => sel('Mid', 'ville creek glen ton'),
        () => sel('Black', 'rock bill ton'),
        () => sel('Yellow', 'oak tree field'),
        () => sel('Green', 'oak tree forest field burr'),
        () => sel('Hollow', 'vale fell plain'),
    ]);

    const props = {
        title: nameFn(),
        size: rng.select(['small', 'medium', 'large']),
        population: 2 + rng.d10(),
        locations: 2 + rng.d6(),
    };

    return {
        name: 'Town',
        seed,
        id: 'town',
        type: 'area',
        props,
    };
}
