import React, { JSX } from 'react';
import { randomSeeded, Prng } from '@std/random';

function Flex({
    row,
    col,
    flex,
    g,
    gap,
    align,
    h,
    minHeight,
    m,
    children,
}: {
    row?: boolean;
    col?: boolean;
    flex?: React.CSSProperties['flex'];
    g?: React.CSSProperties['flexGrow'];
    gap?: React.CSSProperties['gap'];
    align?: React.CSSProperties['alignItems'];
    h?: React.CSSProperties['height'];
    minHeight?: React.CSSProperties['minHeight'];
    m?: React.CSSProperties['margin'];
    children?: React.ReactNode;
}): JSX.Element {
    const style: React.CSSProperties = {
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
        gap: '2px',
    };
    if (row) {
        style.flexDirection = 'row';
        style.alignItems = 'center';
        style.flexGrow = 1;
    }
    if (col) {
        style.flexDirection = 'column';
        style.alignItems = 'stretch';
    }
    if (flex !== undefined) {
        style.flex = flex;
    }
    if (g !== undefined) {
        style.flexGrow = g;
    }
    if (align !== undefined) {
        style.alignItems = align;
    }
    if (gap !== undefined) {
        style.gap = gap;
    }
    if (h !== undefined) {
        style.height = h;
    }
    if (minHeight !== undefined) {
        style.minHeight = minHeight;
    }
    if (m !== undefined) {
        style.margin = m;
    }

    return <div style={style}>{children}</div>;
}

class RNG {
    _rng: Prng;
    constructor(seed: number) {
        this._rng = randomSeeded(BigInt(seed));
    }

    static make_seed8k(): number {
        const seed = Math.floor(Math.random() * Date.now()) % 8192;
        return seed;
    }

    value() {
        const value = this._rng();
        return value;
    }

    bool(): boolean {
        return this._rng() >= 0.5;
    }

    // Exclusive range!
    rangei(min: number, max: number): number {
        const v = (max - min) * this._rng();
        return Math.floor(v) + min;
    }

    coin(): 'heads' | 'tails' {
        return this.bool() ? 'heads' : 'tails';
    }

    d4(): number {
        return 1 + this.rangei(0, 4);
    }
    d6(): number {
        return 1 + this.rangei(0, 6);
    }
    d8(): number {
        return 1 + this.rangei(0, 8);
    }
    d10(): number {
        return 1 + this.rangei(0, 10);
    }
    d12(): number {
        return 1 + this.rangei(0, 12);
    }
    d20(): number {
        return 1 + this.rangei(0, 20);
    }
    d100(): number {
        return 1 + this.rangei(0, 100);
    }

    selectWeighted<T>(arr: [number, T][]): T {
        const total = arr.reduce((acc, [weight]) => acc + weight, 0);
        const r = this.rangei(0, total);
        let sum = 0;
        for (const [weight, value] of arr) {
            sum += weight;
            if (r < sum) {
                return value;
            }
        }
        return arr[arr.length - 1][1];
    }
}

const table_yes_no: [number, string][] = [
    [100, 'exceptional yes'], //
    [1000, 'yes'], //
    [100, 'ambiguous'],
    [1000, 'no'], //
    [100, 'exceptional no'],
];

const table_time_of_day: [number, string][] = [
    [1000, 'early morning'], //
    [1000, 'morning'], //
    [1000, 'early afternoon'], //
    [1000, 'afternoon'], //
    [1000, 'evening'], //
    [1000, 'late evening'], //
    [1000, 'night'], //
    [1000, 'late night'], //
];

const table_weather: [number, string][] = [
    [1000, 'clear'], //
    [1000, 'sunny'], //
    [1000, 'clear'], //
    [250, 'humid'],
    [500, 'light breeze'],
    [500, 'windy'],
    [500, 'light fog'],
    [1000, 'foggy'], //
    [1000, 'cloudy'], //
    [1000, 'partly cloudy'], //
    [500, 'raining'],
    [250, 'drizzling rain'],
    [500, 'snowing'],
    [500, 'heavy snow'],
    [250, 'hailing'],
    [500, 'thunderstorm'],
    [250, 'blizzard'],
];

const table_location: [number, string][] = [
    [1000, 'forest'], //
    [500, 'dense forest'],
    [500, 'forest path'],
    [500, 'forest clearing'],
    [500, 'forest meadow'],
    [500, 'forest creek'],
    [1000, 'grassy hill'],
    [500, 'grassy plain'],
    [1000, 'plain'], //
    [1000, 'crags'],
    [1000, 'mountain'], //
    [500, 'mountain pass'],
    [500, 'clear mountain trail'],
    [500, 'rocky mountain trail'],
    [1000, 'mountain peak'],
    [250, 'desert'], //
    [1000, 'swamp'], //
    [1000, 'tundra'], //
    [250, 'jungle'], //
    [1000, 'cave'], //
    [2500, 'ruins'], //
    [250, 'city'], //
    [500, 'bay'],
    [500, 'lake'],
    [1000, 'river'],
    [500, 'crossroads'],
    [1000, 'town'], //
    [1000, 'village'], //
    [1000, 'road'], //
    [500, 'creek'], //
    [250, 'bridge'],
    [1000, 'river'], //
    [1000, 'lake'], //
    [1000, 'sea'], //
    [1000, 'ocean'], //
];

const table_target: [number, string][] = [
    [1000, 'the region'],
    [1000, 'the area'],
    [1000, 'the location'],
    [1000, 'the party'],
    [500, 'the protagonist'],
    [500, 'the character'],
];

const table_events: [number, string][] = [
    [250, "there's a poor wanderer resting by the wayside"],
    [250, "there's a note posted for a missing person"],
    [500, "there's smoke in the air"],
    [500, "there's an encampment of travelers stopped for a meal"],
    [1000, "there's a large animal stalking behind the party"],
    [1000, "there's what looks to be a ghost in the distance"],
    [1000, 'an old friend appears'],
    [200, 'a distant relative appears'],
    [1000, 'an old acquaintance from the past and not well known appears'],
    [1000, 'a young child running by laughing'],
    [50, "there's a dead body"],
    [250, "there's a blood everywhere"],
    [250, "there's a pool of blood"],
    [1000, "there's expensive jewlery laying on the ground"],
    [1000, "there's a roll of thunder"],
    [250, 'a stranger approaches mistaking your identity'],
    [100, 'a distant sad memory returns unexpectedly into your memory'],
    [500, 'a strikingly beautiful person walking past'],
    [250, 'you trip and suffer a small injury'],
    [200, 'a beggar asks for money'],
    [500, 'there is a local festival going on'],
    [500, 'royalty is passing through'],
    [500, 'it is the birthday of a royal child'],
    [100, 'it is a local holiday in honor of soldiers'],
    [100, 'it is a local holiday in honor of ancient gods'],
    [100, 'it is a local holiday in honor of the harvest'],
    [100, 'it is a day of remembrance for a local hero'],
    [1000, 'a loud noise disturbs all around'],
    [250, 'there is a foul smell nearby'],
    [500, "there's refuse and garbage scattered around"],
    [500, 'there is a small homemade shrine'],
    [500, "there's an abandoned cart"],
    [500, "there's an abandoned campsite"],
    [500, "there's a library"],
    [500, 'the party has been lied to'],
];

// The motivation is a concrete, immediate need: if a motivation is met,
// it is accomplished (e.g. the debt is fully paid) and if it is not,
// something equal and opposite will happen (e.g. the debt will be doubled
// or the collector is sent to hurt them).
//
// This is different from the "values", which are more abstract motivations
// answer "why" they are motivated or the viewport they hold as to how'll
// they'll act.
const table_motivation: [number, string][] = [
    [500, 'as penance'],
    [500, 'as a favor'],
    [500, 'as punishment for a crime'],
    [1000, 'to pay a debt'],
    [1000, 'to earn the favor of someone important'],
    [1000, 'to gain access to a group'],
    [1000, 'to solve a mystery'],
    [500, 'to prevent a murder'],
    [500, 'to prevent a theft'],
    [500, 'to prevent a falsehood'],
    [500, 'to prevent loss of a truth'],
    [500, 'to ensure their motivation'],
    [500, 'to protect their party'],
    [500, 'to protect their family'],
    [500, 'to learn more'],
];

const table_obstacles: [number, string][] = [
    [1000, "it's broken"],
    [1000, "it's locked"],
    [1000, "it's only part of what is needed"],
    [1000, "it's stolen"],
    [1000, "it's missing"],
    [1000, "it's stuck in place"],
    [1000, 'fear of being recognized'],
    [1000, "there's an old enemy there"],
    [500, "there's a old lost-love there"],
    [1000, 'it was a trick and lie'],
    [1000, 'no one will listnen to them'],
    [1000, 'it will not be understood by others'],
    [1000, 'it will be seen as a betrayal'],
    [1000, 'it will be seen as a crime'],
    [1000, 'it will be seen as a mistake'],
    [1000, 'they lack the skill'],
    [1000, 'they lack the resources'],
    [1000, 'they lack the time'],
];

const table_actions: [number, string][] = [
    [1000, 'hide'],
    [1000, 'steal'],
    [1000, 'return'],
    [1000, 'destroy'],
    [1000, 'create'],
    [1000, 'protect'],
    [1000, 'share'],
];

const table_values: [number, string][] = [
    [1000, 'fairness'],
    [1000, 'fame'],
    [1000, 'family'],
    [1000, 'freedom'],
    [1000, 'friendship'],
    [1000, 'glory'],
    [1000, 'fun'],
    [1000, 'honor'],
    [1000, 'independence'],
    [1000, 'justice'],
    [1000, 'knowledge'],
    [1000, 'love'],
    [1000, 'loyalty'],
    [1000, 'lust'],
    [1000, 'order'],
    [1000, 'novelty'],
    [1000, 'peace'],
    [1000, 'power'],
    [1000, 'religion'],
    [1000, 'resources'],
    [1000, 'revenge'],
    [1000, 'safety'],
    [1000, 'self-respect'],
    [1000, 'skill'],
    [1000, 'tradition'],
    [1000, 'truth'],
    [1000, 'wealth'],
];

const table_feeling: [number, string][] = [
    [1000, 'abandonment'],
    [1000, 'anger'],
    [1000, 'anxiety'],
    [1000, 'coersion'],
    [1000, 'comfort'],
    [1000, 'confusion'],
    [1000, 'disappointment'],
    [1000, 'exhaustion'],
    [1000, 'fear'],
    [1000, 'helplessness'],
    [1000, 'in perfect flow'],
    [1000, 'insecurity'],
    [1000, 'joy'],
    [1000, 'opportunity'],
    [1000, 'optimism'],
    [1000, 'out of control'],
    [1000, 'panic'],
    [1000, 'pessimism'],
    [1000, 'pride'],
    [1000, 'protectiveness'],
    [1000, 'shock'],
    [1000, 'strength'],
    [1000, 'surprise'],
    [1000, 'solitude'],
    [1000, 'foolish'],
    [1000, 'disgust'],
];

const table_threats: [number, string][] = [
    [1000, 'bandits'],
    [1000, 'looters'],
    [1000, 'a fever'],
    [1000, 'debt'],
    [1000, 'homelessness'],
    [1000, 'hunger'],
    [1000, 'cold'],
    [1000, 'sickness'],
    [1000, 'solitude'],
    [1000, 'a pandemic'],
    [1000, 'an inherited debt that must be paid'],
    [1000, 'an incurred debt that must be paid'],
    [1000, 'an antagonist knows a powerful secret about the character'],
    [1000, 'the character has a shameful secret'],
    [1000, 'debilitating illness'],
    [250, 'debilitating nightmares'],
    [500, 'insomnia'],
    [1000, 'a murder on the loose'],
    [1000, 'a serial killer on the loose'],
    [1000, 'climate change'],
    [1000, 'drought'],
    [1000, 'flooding'],
    [1000, 'an earthquake'],
    [1000, 'a tornado'],
    [1000, 'a sense of hopelessness'],
    [1000, 'dangerous political divides'],
    [1000, 'torrential rains'],
    [1000, 'hail of dangerous size'],
    [1000, 'a freezing snowstorm'],
    [1000, 'a blinding snowstorm'],
    [1000, 'a lightning storm'],
    [1000, 'an avalanche'],
    [1000, 'instability in the local government'],
    [1000, 'a dangerous election in progress'],
    [500, 'a recent assisination'],
    [1000, 'rampant gang warfare'],
    [1000, 'guild price-gouging'],
    [1000, 'a hurricane'],
    [1000, 'a tsunami'],
    [1000, 'a meteor strike'],
    [1000, 'a volcanic eruption'],
    [1000, 'a massive factory accident'],
    [1000, 'a plague'],
    [1000, 'a war'],
    [1000, 'famine'],
    [1000, 'major legal case about to be decided'],
    [1000, 'violence against minorities'],
    [1000, 'a political rebellion'],
    [1000, 'a military rebellion'],
    [1000, 'a government strike'],
    [1000, 'a farmers strike'],
    [1000, 'a coup'],
    [1000, 'a wildfire'],
    [1000, 'a hunt for a thief'],
    [1000, 'a hunt for a murderer'],
    [250, 'an infestation of insects'],
    [1000, 'the water has been poisoned'],
    [500, 'reports of a mass killing in the area'],
];

export function App(): JSX.Element {
    const [seed, setSeed] = React.useState(RNG.make_seed8k());

    return (
        <Flex col m="12px 64px">
            <Flex row align="end" m="0 0 12px">
                <h1 style={{ margin: '0 12px 0 0' }}>Tables {seed}</h1>
                <Flex row m="0 0 4px">
                    <a
                        href="#"
                        onClick={(evt) => {
                            evt.preventDefault();
                            evt.stopPropagation();
                            setSeed(RNG.make_seed8k());
                        }}
                    >
                        re-roll
                    </a>
                </Flex>
            </Flex>

            <DiceTable seed={seed} />
            <Table seed={seed} />
        </Flex>
    );
}

function DiceTable({ seed }: { seed: number }): JSX.Element {
    const rng = new RNG(seed);

    const table: { [k: string]: number | string } = {
        d4: rng.d4(), //
        d6: rng.d6(), //
        d8: rng.d8(), //
        d10: rng.d10(), //
        d12: rng.d12(), //
        d20: rng.d20(), //
        d100: rng.d100(), //
        coin: rng.coin(), //
        'yes/no': rng.selectWeighted(table_yes_no), //
    };

    const Row = ({ k }: { k: string }) => (
        <Flex row>
            <Flex flex="0 0 5em">{k}</Flex>
            <Flex>{table[k]}</Flex>
        </Flex>
    );

    return (
        <Flex row m="0 0 32px 0" align="start">
            <Flex col flex="0 0 14em">
                <Row k="d4" />
                <Row k="d6" />
                <Row k="d8" />
                <Row k="d10" />
                <Row k="d12" />
            </Flex>
            <Flex col flex="0 0 14em">
                <Row k="d20" />
                <Row k="d100" />
                <Row k="coin" />
                <Row k="yes/no" />
            </Flex>
        </Flex>
    );
}

function Table({ seed }: { seed: number }): JSX.Element {
    const rng = new RNG(seed);

    const table: [string, number | string][] = [
        ['time of day', rng.selectWeighted(table_time_of_day)],
        ['weather', rng.selectWeighted(table_weather)],
        ['location', rng.selectWeighted(table_location)],
        ['threat', rng.selectWeighted(table_threats)],
        ['target', rng.selectWeighted(table_target)],
        ['event', rng.selectWeighted(table_events)],
        ['action', rng.selectWeighted(table_actions)],
        ['motivation', rng.selectWeighted(table_motivation)],
        ['rationale', rng.selectWeighted(table_values)],
        ['feeling', rng.selectWeighted(table_feeling)],
        ['obstacle', rng.selectWeighted(table_obstacles)],
        ['yes/no', rng.selectWeighted(table_yes_no)], //
    ];

    const v: { [k: string]: string } = {};
    for (const [name, value] of table) {
        const key = name.replace(/[ -\/]/g, '_').toLowerCase();
        v[key] = `${value}`;
    }

    const V = ({ k }: { k: string }) => <strong style={{ color: '#156785' }}>{v[k]}</strong>;

    return (
        <Flex col>
            <h3>Scene template</h3>
            <Flex row gap={32} align="start">
                <Flex col flex="0 0 40em">
                    {table.map(([name, value], index) => (
                        <Flex key={name || index} minHeight="1em">
                            <Flex flex="0 0 10em">{name}</Flex>
                            <Flex>{value}</Flex>
                        </Flex>
                    ))}
                </Flex>
                <Flex col g={0}>
                    <div style={{ maxWidth: '50em' }}>
                        It is <V k="time_of_day" /> and the weather is <V k="weather" />. The party
                        is at a <V k="location" />. There is <V k="threat" /> threating{' '}
                        <V k="target" />. The party sees <V k="event" />. The party knows they must
                        act <V k="motivation" /> so, driven by a sense of <V k="rationale" /> and
                        shaped by a feeling of <V k="feeling" />, they decide to <V k="action" />.
                        However, there's the problem that <V k="obstacle" />. Will it work:{' '}
                        <V k="yes_no" />.
                    </div>
                </Flex>
            </Flex>
        </Flex>
    );
}
