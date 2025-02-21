import React, { JSX } from 'react';
import { RNG } from './raiment-core/index.ts';
import { D, Div, css } from './raiment-ui/index.ts';
import { TownView } from './views/town_view.tsx';
import { useGitHubAuthToken, useGitHubAPI } from './raiment-ui/use_github_auth.ts';
import yaml from 'js-yaml';
import lodash from 'lodash';
import { nanoid } from 'nanoid';
import { EventEmitter } from './raiment-core/index.ts';
import { DrawRegionView } from './views/region_view.tsx';
import { World, JournalEntry, Action } from './world.ts';

function Console({
    seed,
    world,
    addEntry,
}: {
    seed: number;
    world: World;
    addEntry: (props: { [k: string]: any }) => void;
}): JSX.Element {
    const [rng] = React.useState(new RNG(seed));

    const handler =
        (type: string, props: { [k: string]: any } = {}) =>
        () =>
            addEntry({ type, seed: rng.d8192(), ...props });

    return (
        <D
            css={css`
                .this {
                    margin-top: 32px;
                    input {
                        font-family: inherit;
                        font-size: inherit;
                    }
                }
            `}
        >
            <input
                id="console-input"
                type="text"
                placeholder="Enter a command (WARNING: Not yet implemented. Use buttons below instead.)"
                style={{
                    width: '100%',
                }}
            />
            <Flex row>
                <button onClick={handler('dice')}>roll dice</button>
                <button onClick={handler('scene')}>scene</button>
                <button onClick={handler('region')}>region</button>
                <button onClick={handler('area')}>area</button>
                <button onClick={handler('location')}>location</button>
                <button onClick={handler('item')}>item</button>
                <button onClick={handler('npc')}>NPC</button>
                <button onClick={handler('character')}>character</button>
                <button onClick={handler('town')}>town</button>
            </Flex>
            <div id="console-focus-anchor" style={{ height: 8 }} />
        </D>
    );
}

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
    style,
    className,
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
    style?: React.CSSProperties;
    className?: string;
    children?: React.ReactNode;
}): JSX.Element {
    const computed: React.CSSProperties = {
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
        gap: '2px',
    };
    if (row) {
        computed.flexDirection = 'row';
        computed.alignItems = 'center';
        computed.flexGrow = 1;
    }
    if (col) {
        computed.flexDirection = 'column';
        computed.alignItems = 'stretch';
    }
    if (flex !== undefined) {
        computed.flex = flex;
    }
    if (g !== undefined) {
        computed.flexGrow = g;
    }
    if (align !== undefined) {
        computed.alignItems = align;
    }
    if (gap !== undefined) {
        computed.gap = gap;
    }
    if (h !== undefined) {
        computed.height = h;
    }
    if (minHeight !== undefined) {
        computed.minHeight = minHeight;
    }
    if (m !== undefined) {
        computed.margin = m;
    }
    if (style !== undefined) {
        Object.assign(computed, style);
    }

    return (
        <div className={className} style={computed}>
            {children}
        </div>
    );
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

const table_area: [number, string][] = [
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
    const widthRules = [2, 4, 8, 16, 24, 32, 48, 64, 72, 96, 128, 192, 256, 384, 512]
        .map((n) => `.w-${n} { width: ${n}px; }`)
        .join('\n');
    const gapRules = [2, 4, 8, 16, 32, 64].map((n) => `.gap-${n} { gap: ${n}px; }`).join('\n');

    return (
        <D
            css={css`
                .global {
                    input,
                    select,
                    option,
                    div {
                        box-sizing: border-box;
                    }

                    .flex-row {
                        display: flex;
                        flex-direction: row;
                        align-items: center;
                    }
                    ${gapRules}
                    ${widthRules}

                    .bold {
                        font-weight: bold;
                    }

                    .link {
                        color: #33f;
                        cursor: pointer;
                        &:hover {
                            color: #a3f;
                        }
                    }
                }
            `}
        >
            <ContentGate />
        </D>
    );
}

function ContentGate(): JSX.Element {
    const accessToken = useGitHubAuthToken();
    return accessToken ? <App2 /> : <GitHubSignIn />;
}

function GitHubSignIn() {
    const isLocalAuth = window.location.hostname === 'localhost';
    const clientID = isLocalAuth ? 'Ov23lilAyyeHVnqZ1pGc' : 'Ov23li89ZvKkoY3YqFDj';
    const paramsHash = {
        scope: 'read:user, repo, gist',
        client_id: clientID,
        state: encodeURIComponent(window.location.href),
        allow_signup: 'false',
        prompt: 'select_account',
    };
    const params = new URLSearchParams(paramsHash);
    const url = `https://github.com/login/oauth/authorize?${params}`;

    return (
        <D
            css={css`
                .self {
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    margin: 64px auto;

                    .button {
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        gap: 32px;

                        padding: 8px 64px;
                        background: #000;
                        color: white;
                        border-radius: 12px;
                        line-height: 1.5;

                        cursor: pointer;

                        img {
                            width: 32px;
                            height: 32px;
                        }

                        a {
                        }
                    }
                }
            `}
        >
            <div
                className="button"
                onClick={() => {
                    window.location.assign(url);
                }}
            >
                <div>Sign in with GitHub</div>
            </div>
        </D>
    );
}

function updateRoute(route: string) {
    const url = new URL(window.location.href);
    url.searchParams.set('route', route);
    history.replaceState(null, '', url.toString());
    const event = new CustomEvent('routechange', {});
    window.dispatchEvent(event);
}

function TopNav(): JSX.Element {
    const api = useGitHubAPI();
    const [user, setUser] = React.useState<any | null>(null);

    React.useEffect(() => {
        if (!api) {
            return;
        }
        const go = async () => {
            setUser(await api.user());
        };
        go();
    }, [api]);

    return (
        <D
            css={css`
                .self {
                    margin-bottom: 32px;
                }
            `}
        >
            <D
                css={css`
                    .self {
                        box-sizing: border-box;
                        position: fixed;
                        display: flex;
                        width: 100vw;
                        flex-direction: row;
                        padding: 2px 16px;
                        border-bottom: 1px solid #ccc;
                        gap: 64px;
                        background-color: white;
                    }
                `}
            >
                <D cl="flex-row bold">snowtale (prototype v0.1)</D>

                <D cl="flex-row gap-16">
                    <D
                        cl="link"
                        onClick={() => {
                            updateRoute('journal');
                        }}
                    >
                        journal
                    </D>
                    <D
                        cl="link"
                        onClick={() => {
                            updateRoute('tables');
                        }}
                    >
                        tables
                    </D>
                </D>

                <D style={{ flexGrow: 1 }} />
                <D
                    css={css`
                        .self {
                            display: flex;
                            flex-direction: row;
                            align-items: center;
                            gap: 16px;
                        }
                    `}
                >
                    {user && (
                        <Flex row gap={8}>
                            <img
                                src={user.avatar_url}
                                style={{
                                    height: 16,
                                    width: 16,
                                }}
                            />
                            {user.login}
                        </Flex>
                    )}
                    <button
                        onClick={() => {
                            localStorage.removeItem('github_auth/access_token');
                            window.location.reload();
                        }}
                    >
                        sign out
                    </button>
                </D>
            </D>
        </D>
    );
}

type Entry = { [k: string]: any };

function useRenderOnWindowEvent(event: string) {
    const [_incarnation, setIncarnation] = React.useState(0);
    React.useEffect(() => {
        const handler = (evt: any) => {
            setIncarnation((i) => i + 1);
        };
        window.addEventListener(event, handler);
        return () => {
            window.removeEventListener(event, handler);
        };
    }, []);
}

function useRenderOnEvent(obj: any, event: string) {
    const [_incarnation, setIncarnation] = React.useState(0);
    React.useEffect(() => {
        const handler = () => {
            setIncarnation((i) => i + 1);
        };
        return obj.events.on(event, handler);
    }, []);
}

type EncyclopediaData = {
    protagonist: ProtagonistData;
    entries: Entry[];
};

type ProtagonistData = {
    name: string;
};

class Protagonist {
    data: ProtagonistData;

    constructor(data: ProtagonistData) {
        this.data = data;
    }
}

class Encyclopedia {
    events: EventEmitter = new EventEmitter();
    data: EncyclopediaData = {
        protagonist: {
            name: '',
        },
        entries: [],
    };

    modify(params: Partial<EncyclopediaData>) {
        Object.assign(this.data, params);
        this.events.fire('modified');
    }
    update(cb: (data: EncyclopediaData) => void) {
        cb(this.data);
        this.events.fire('modified');
    }
}

export function App2(): JSX.Element {
    const [encyclopedia] = React.useState<Encyclopedia>(new Encyclopedia());
    return <App3 encyclopedia={encyclopedia} />;
}

export function App3({ encyclopedia }: { encyclopedia: Encyclopedia }): JSX.Element {
    useRenderOnWindowEvent('routechange');

    const url = new URL(window.location.href);
    const route = url.searchParams.get('route') || 'journal';

    return (
        <Div
            css={css`
                .self {
                    display: flex;
                    flex-direction: column;
                    align-items: stretch;

                    flex-grow: 1;
                    width: 100%;
                    height: 100vh;
                }
            `}
        >
            <TopNav />
            <Div
                css={css`
                    .self {
                    }
                `}
            >
                {route === 'tables' ? <Tables /> : <Journal encyclopedia={encyclopedia} />}
            </Div>
        </Div>
    );
}

type NameEntry = {
    id: string;
    created: number;
    name: string;
    rarity: number; // 1 (rare) to 1000 (common)
    unique: boolean; // can only be played once per world
};

type NamesTable = {
    entries: NameEntry[];
};

function Tables(): JSX.Element {
    type SortOrder = {
        field: keyof NameEntry;
        reverse: boolean;
    };

    const [sort, setSort] = React.useState<SortOrder>({ field: 'created', reverse: false });
    const [table, setTable] = React.useState<NamesTable | null>(null);
    const api = useGitHubAPI();

    React.useEffect(() => {
        if (!api) {
            return;
        }
        const go = async () => {
            const raw = await api.readFileContents('snowfall/table-names.yaml');
            const obj: NamesTable = raw ? yaml.load(raw) : { entries: [] };

            for (const entry of obj.entries) {
                entry.id ??= nanoid();
                entry.created ??= Date.now();
            }
            setTable(obj);
        };
        go();
    }, [api]);

    React.useEffect(() => {
        if (!table) {
            return;
        }
        const entries = [...table.entries];
        entries.sort((a: NameEntry, b: NameEntry) => {
            switch (sort.field) {
                case 'created':
                    return a.created - b.created;
                case 'name': {
                    return a.name.localeCompare(b.name);
                }
                case 'rarity': {
                    return a.rarity - b.rarity;
                }
            }
            return 0;
        });
        if (sort.reverse) {
            entries.reverse();
        }
        setTable({
            ...table,
            entries,
        });
    }, [sort]);

    if (!api || !table) {
        return <div>loading...</div>;
    }

    const updateHandler = (cb: Function) => {
        return (evt: any) => {
            const value = evt.target.value;
            const updated = lodash.cloneDeep(table);
            cb(updated, value);
            api.updateFileContents('snowfall/table-names.yaml', yaml.dump(updated));
            setTable(updated);
        };
    };

    const updateSort = (field: keyof NameEntry) => {
        return () => {
            setSort((s) => {
                return {
                    field,
                    reverse: s.field === field ? !s.reverse : false,
                };
            });
        };
    };

    return (
        <D
            css={css`
                .self {
                    margin: 12px 32px;

                    border: 1px solid #ccc;
                    border-radius: 8px;
                    padding: 16px;

                    width: 800px;

                    * {
                        box-sizing: border-box;
                        background-color: transparent;
                    }

                    input {
                        box-sizing: border-box;
                        font-family: inherit;
                        font-size: inherit;
                        width: 100%;
                        padding: 2px 4px;
                    }

                    input,
                    select {
                        border: 1px dotted transparent;
                        border-radius: 4px;

                        &:hover {
                            border-color: rgba(83, 24, 192, 0.434);
                        }
                        &:focus {
                            border-color: rgba(23, 24, 252, 0.834);
                        }

                        transition-property: border-color;
                        transition-delay: 40ms;
                        transition-duration: 300ms;
                    }
                    select,
                    option {
                        text-align: right;
                    }

                    .header-row {
                        font-weight: bold;
                        margin-bottom: 4px;
                        border-bottom: solid 1px rgba(0, 0, 0, 0.1);

                        * {
                            cursor: pointer;

                            &:hover {
                                color: blue;
                            }
                        }
                    }
                    .table-row {
                        &:focus-within {
                            background-color: rgba(110, 175, 241, 0.1);
                        }
                    }
                }
            `}
        >
            <D cl="flex-row gap-4 header-row">
                <D cl="w-24"></D>
                <D cl="w-384" onClick={updateSort('name')}>
                    Name
                </D>
                <D
                    cl="w-64"
                    css={css`
                        .self {
                            cursor: pointer;

                            &:hover {
                                color: blue;
                            }
                        }
                    `}
                    onClick={updateSort('rarity')}
                >
                    Rarity
                </D>
                <D cl="w-128">Flags</D>
            </D>

            {table.entries.map((entry, i) => (
                <D key={entry.id} cl="flex-row gap-4 table-row">
                    <D cl="w-24" style={{ textAlign: 'right' }}>
                        {i + 1}
                    </D>
                    <D cl="w-384">
                        <input
                            id={`table-name-${i}`}
                            type="text"
                            value={entry.name}
                            onChange={(evt) => {
                                const updated = lodash.cloneDeep(table);
                                const entry = updated.entries.find(
                                    (e) => e.id === table.entries[i].id
                                )!;
                                entry.name = evt.target.value;
                                api.updateFileContents(
                                    'snowfall/table-names.yaml',
                                    yaml.dump(updated)
                                );
                                setTable(updated);
                            }}
                            onKeyDown={(evt) => {
                                if (evt.key === 'ArrowUp') {
                                    evt.preventDefault();
                                    evt.stopPropagation();
                                    document.getElementById(`table-name-${i - 1}`)?.focus();
                                }
                                if (evt.key === 'ArrowDown') {
                                    evt.preventDefault();
                                    evt.stopPropagation();
                                    document.getElementById(`table-name-${i + 1}`)?.focus();
                                }
                            }}
                        />
                    </D>
                    <D cl="w-64">
                        <select
                            value={entry.rarity}
                            onChange={updateHandler((updated: NamesTable, value: string) => {
                                const entry = updated.entries.find(
                                    (e) => e.id === table.entries[i].id
                                )!;
                                entry.rarity = parseInt(value, 10);
                            })}
                        >
                            <option value={1000}>1000</option>
                            <option value={500}>500</option>
                            <option value={250}>250</option>
                            <option value={100}>100</option>
                            <option value={50}>50</option>
                            <option value={10}>10</option>
                            <option value={1}>1</option>
                        </select>
                    </D>
                    <D cl="w-128">{entry.unique ? 'unique' : ''}</D>
                </D>
            ))}
            <D
                css={css`
                    .self {
                        margin: 16px 0;
                    }
                `}
            >
                <button
                    onClick={updateHandler((updated: NamesTable, value: string) => {
                        const entry = {
                            id: nanoid(),
                            created: Date.now(),
                            name: 'New name',
                            rarity: 1000,
                            unique: false,
                        };
                        updated.entries.push(entry);
                    })}
                >
                    add
                </button>
            </D>
        </D>
    );
}

function Journal({ encyclopedia }: { encyclopedia: Encyclopedia }): JSX.Element {
    const [seed, setSeed] = React.useState(RNG.make_seed8k());
    const rng = React.useMemo(() => new RNG(seed), [seed]);
    const world = React.useMemo(() => new World(seed), [seed]);

    useRenderOnEvent(encyclopedia, 'modified');
    useRenderOnEvent(world, 'modified');

    const addEntry = React.useCallback(
        (e: Entry) => {
            encyclopedia.update(({ entries }) => {
                entries.push(e);
            });
        },
        [encyclopedia]
    );

    React.useEffect(() => {
        return world.events.on('modified', () => {
            world.runActions();
        });
    }, [world]);

    React.useEffect(() => {
        const el = document.querySelector('#console-input') as HTMLInputElement;
        const el2 = document.querySelector('#console-focus-anchor') as HTMLDivElement;
        if (el) {
            el.focus();
            el2?.scrollIntoView({ behavior: 'smooth', block: 'end', inline: 'end' });
        }
    }, [encyclopedia.data.entries.length]);

    return (
        <Div
            css={css`
                .self {
                    display: flex;
                    flex-direction: column;
                    padding: 12px 64px;
                    flex-grow: 1;
                    justify-content: start;
                    height: calc(100vh - 32px);
                    max-height: calc(100vh - 32px);
                    min-height: calc(100vh - 32px);
                }
            `}
        >
            <Flex row align="end" m="0 0 12px" style={{ flexGrow: 0 }}>
                <h1 style={{ margin: '0 12px 0 0' }}>Galthea seed {seed}</h1>
                <Flex row m="0 0 4px">
                    <a
                        href="#"
                        onClick={(evt) => {
                            evt.preventDefault();
                            evt.stopPropagation();
                            setSeed(RNG.make_seed8k());
                        }}
                    >
                        reroll
                    </a>
                </Flex>
            </Flex>
            <Div
                css={css`
                    .self {
                        display: flex;
                        flex-direction: column;
                        flex-grow: 1;
                        flex-shrink: 0;
                    }
                `}
            >
                <Div
                    css={css`
                        .self {
                        }
                    `}
                >
                    <div style={{ height: 32 }} />
                    {world.journal.map((entry, i) => (
                        <div
                            key={i}
                            style={{
                                padding: 2,
                                border: '1px solid #aac',
                                borderRadius: 4,
                            }}
                        >
                            <Flex row>
                                <div style={{ opacity: 0.4, marginRight: 32 }}>{i + 1}</div>
                                <div style={{ flexGrow: 1 }}>
                                    <JournalEntryView world={world} entry={entry} />
                                </div>
                            </Flex>
                        </div>
                    ))}
                    <div style={{ height: 32 }} />
                    {[...encyclopedia.data.entries].map((entry, i) => (
                        <div
                            key={i}
                            style={{
                                padding: 2,
                                border: '1px solid #ccc',
                                borderRadius: 4,
                            }}
                        >
                            <Flex row>
                                <div style={{ opacity: 0.4, marginRight: 32 }}>{i + 1}</div>
                                <div style={{ flexGrow: 1 }}>
                                    <EntryView entry={entry} />
                                </div>
                            </Flex>
                        </div>
                    ))}
                </Div>
            </Div>
            <Console seed={rng.d8192()} world={world} addEntry={addEntry} />
        </Div>
    );
}

function JournalEntryView({ world, entry }: { world: World; entry: JournalEntry }): JSX.Element {
    const handleClick = (url: string) => {
        url = url.trim();
        const parts = url.split(/\s+/);
        const pairs = parts.map((part) => part.split(':').map((s) => s.trim()));

        if (pairs[0][0] !== 'action') {
            return;
        }

        const args: { [k: string]: any } = {};
        for (let i = 1; i < pairs.length; i++) {
            let value: any = pairs[i][1];
            if (value.match(/^[0-9]+$/)) {
                value = parseInt(value, 10);
            } else if (value.match(/^[0-9]+\.[0-9]+$/)) {
                value = parseFloat(value);
            }
            args[pairs[i][0]] = value;
        }

        const action: Action = {
            type: pairs[0][1] as Action['type'],
            selector: args,
        } as Action;
        world.enqueue(action);
    };

    switch (entry.type) {
        case 'markdown':
            return (
                <div style={{ margin: '8px 0' }}>
                    <Markdown content={entry.content} onClickLink={handleClick} />
                </div>
            );
        case 'draw_region':
            return <DrawRegionView world={world} entry={entry} />;
        default:
            return <div>{JSON.stringify(entry)}</div>;
    }
}

function EntryView({ entry }: { entry: Entry }): JSX.Element {
    switch (entry.type) {
        case 'markdown':
            return (
                <div style={{ margin: '8px 0' }}>
                    <Markdown content={entry.content} />
                </div>
            );
        case 'dice':
            return <DiceTable seed={entry.seed} />;
        case 'scene':
            return <SceneTable seed={entry.seed} />;
        case 'character':
            return <CharacterTable seed={entry.seed} />;
        case 'town':
            return <TownView seed={entry.seed} />;
        default:
            return <div>{JSON.stringify(entry)}</div>;
    }
}

function Markdown({
    content,
    onClickLink,
}: {
    content: string;
    onClickLink?: (url: string) => void;
}): JSX.Element {
    const blocks = content.split('\n\n');
    while (blocks.length > 0 && blocks[0].trim() === '') {
        blocks.shift();
    }
    while (blocks.length > 0 && blocks[blocks.length - 1].trim() === '') {
        blocks.pop();
    }

    const Block = ({ text }: { text: string }): JSX.Element => {
        const matchers: [RegExp, Function][] = [
            [/\*\*(.*?)\*\*/, (m: any) => <strong>{m[1]}</strong>],
            [
                /\[(.*?)\]\((.*?)\)/,
                (m: any) => (
                    <a
                        href="#"
                        onClick={(evt: React.MouseEvent<HTMLAnchorElement>) => {
                            evt.preventDefault();
                            evt.stopPropagation();
                            onClickLink?.(m[2]);
                        }}
                    >
                        {m[1]}
                    </a>
                ),
            ],
        ];

        const frags: any[] = [];
        while (text.length > 0) {
            const first: [number, Function | null, any] = [-1, null, []];
            for (const [re, handler] of matchers) {
                const m = text.match(re);
                if (!m || m.index === undefined) {
                    continue;
                }
                if (m.index < first[0] || first[0] === -1) {
                    first[0] = m.index;
                    first[1] = handler;
                    first[2] = m;
                }
            }

            if (first[0] !== -1) {
                const [index, handler, m] = first;
                const prefix = text.slice(0, index);
                if (prefix.length > 0) {
                    frags.push(<span>{prefix}</span>);
                    text = text.slice(prefix.length);
                }
                frags.push(handler!(m));
                text = text.slice(m[0].length);
            } else {
                frags.push(<span>{text}</span>);
                text = '';
            }
        }

        const re = /\*\*(.*?)\*\*/;
        const arr = text.split(re);
        const parts = [];
        parts.push({ type: 'span', text: arr[0] });
        for (let i = 1; i < arr.length; i += 2) {
            parts.push({ type: 'bold', text: arr[i] });
            if (arr[i + 1]) {
                parts.push({ type: 'span', text: arr[i + 1] });
            }
        }

        return (
            <div style={{ margin: '4px 0 12px', maxWidth: '80ch' }}>
                {frags.map((f, i) => (
                    <React.Fragment key={i}>{f}</React.Fragment>
                ))}
            </div>
        );
    };

    return (
        <D
            css={css`
                .self {
                    div {
                        min-height: 1em;
                    }
                }
            `}
        >
            {blocks.map((line, i) => (
                <Block key={i} text={line} />
            ))}
        </D>
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
        <Flex col>
            <Flex row m="0 0 2px 0" align="start">
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
        </Flex>
    );
}

function CharacterTable({ seed }: { seed: number }): JSX.Element {
    const rng = new RNG(seed);

    const values: string[] = rng.shuffle(['independence', 'knowledge', 'security']);

    const table: { [k: string]: [string, string] } = {
        name: ['Kestrel', 'Name'],
        seed: [`${seed}`, 'Seed'],
        primary_value: [values[0], 'Primary value'],
        secondary_value: [values[1], 'Secondary value'],
        trigger_value: [rng.selectWeighted(table_values), 'Trigger value'],
    };

    const Row = ({ k }: { k: string }) => (
        <Flex row>
            <Flex flex="0 0 10em">{table[k][1]}</Flex>
            <Flex>{table[k][0]}</Flex>
        </Flex>
    );

    const Example = ({ children }: { children: React.ReactNode }): JSX.Element => (
        <span style={{ color: '#687' }}>🎲 {children}</span>
    );

    return (
        <Flex col>
            <div style={{ fontWeight: 'bold' }}>Character</div>
            <Flex row gap={32} align="start">
                <Flex col flex="0 0 40em">
                    {Object.entries(table).map(([k]) => (
                        <Row key={k} k={k} />
                    ))}
                </Flex>
                <Flex col g={0}>
                    <div style={{ maxWidth: '50em' }}>
                        <p style={{ marginTop: 0 }}>
                            <strong>Primary value</strong> is something the lens through which the
                            character views every single decision they make. Which action will best
                            support this value they hold?
                            <Example>
                                A character's friend has been dishonored and mocked by the captain
                                of the caravan the party needs to travel with. The character's
                                primary value is "loyalty." From the lens of this value, the need to
                                forgo taking caravan to stay by their friend.
                            </Example>
                        </p>
                        <p>
                            <strong>Secondary value</strong> is the fallback for cases where either
                            (a) it is unclear how an action or trade-off would affect the primary
                            value, or (b) the secondary value clearly applies an obvious choice.{' '}
                            <Example>
                                A character sees ten gold on the table left on the table by a
                                careless merchant who has already left the inn. Their cousin is in
                                prison and they need ten gold more to bail the cousin out before
                                harm comes to her. The character's primary value is "glory" and
                                their second value is "family". The value of "family" is a clear fit
                                to the situation so the characters takes the coins.
                            </Example>
                        </p>
                        <p>
                            The <strong>Trigger value</strong> comes into play when the character
                            sees that value being threatened or violated when it is a postive value
                            or enacted and encouraged when it is a negative value. In such cases,
                            the character <em>must</em> act to stop the offense and does so with{' '}
                            <em>irrational haste and fervor</em>, usually in ways that defy common
                            sense and in a way that damages the character and/or the party's greater
                            goals.{' '}
                            <Example>
                                A character with a trigger value of "strength" sees a man picking on
                                a child; they rush up to stop the man though it will undoubtedly get
                                them beaten up, arrested, and derail the party's plans.
                            </Example>
                        </p>
                    </div>
                </Flex>
            </Flex>
        </Flex>
    );
}

function SceneTable({ seed }: { seed: number }): JSX.Element {
    const rng = new RNG(seed);

    const table: [string, number | string][] = [
        ['time of day', rng.selectWeighted(table_time_of_day)],
        ['weather', rng.selectWeighted(table_weather)],
        ['area', rng.selectWeighted(table_area)],
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
                <Flex col g={0} style={{ maxWidth: '50em' }}>
                    <div>
                        It is <V k="time_of_day" /> and the weather is <V k="weather" />. The party
                        is at a <V k="area" />. There is <V k="threat" /> threating <V k="target" />
                        . The party sees <V k="event" />. The party knows they must act{' '}
                        <V k="motivation" /> so, driven by a sense of <V k="rationale" /> and shaped
                        by a feeling of <V k="feeling" />, they decide to <V k="action" />. However,
                        there's the problem that <V k="obstacle" />. Will it work: <V k="yes_no" />.
                    </div>
                    <p>...</p>
                    <p>
                        <strong>A Scene</strong> is composed, a minimum of:
                        {[
                            '- a time, a place, and the current conditions',
                            '- A broader overarching threat targeting the region, party, character, etc.',
                            '- Some event that forces the party to act',
                            '- A value that tells them how they must act',
                            '- A feeling that shapes how they feel',
                            '- The action they are taking in response to the event',
                            '- An obstable that makes that action more difficult',
                            '- An outcome as to whether the action succeeds',
                        ].map((l) => (
                            <div key={l}>{l}</div>
                        ))}
                    </p>

                    <p>
                        <strong>A Sequence</strong> is a Scene where, if the outcome fails, the
                        failure is a new event demanding a new action motivated by potentially
                        updated values and feelings. The obstacle should change to reflect new
                        event.
                    </p>
                </Flex>
            </Flex>
        </Flex>
    );
}
