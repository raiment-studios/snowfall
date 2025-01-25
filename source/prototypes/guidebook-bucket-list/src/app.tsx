import React, { JSX } from 'react';
import { css, Flex } from './guidebook-ui/index.ts';
import { Documentation } from './documentation.tsx';
import { Database, BucketItem, BucketItemData } from './model.ts';
import { GitHubAPI, useGitHubAPI, useGitHubAuthToken } from './guidebook-ui/use_github_auth.ts';

export function App(): JSX.Element {
    return (
        <Flex col>
            <ContentGate />
        </Flex>
    );
}

function ContentGate(): JSX.Element {
    const accessToken = useGitHubAuthToken();
    return accessToken ? <Content /> : <GitHubSignIn />;
}

function Content(): JSX.Element {
    const [database, setDatabase] = React.useState<Database | null>(null);
    const ghAPI = useGitHubAPI();

    React.useEffect(() => {
        if (!ghAPI) {
            return;
        }
        const go = async () => {
            setDatabase(await Database.load(ghAPI));
        };
        go();
    }, [ghAPI?.token]);

    if (!database) {
        return <p>Loading...</p>;
    }

    return (
        <>
            <TopNavigation database={database} />
            <Flex col m="12px 64px">
                <Flex row>
                    <h1>Bucket List</h1>
                    <div style={{ width: 32 }} />
                    <button
                        onClick={() => {
                            const go = async () => {
                                await database.save();
                                window.location.reload();
                            };
                            go();
                        }}
                    >
                        save
                    </button>
                </Flex>
                <BucketListView database={database} />

                <div>
                    <div
                        style={{
                            margin: '128px 0 32px',
                            borderTop: '1px solid rgba(0,0,0,0.1)',
                            fontStyle: 'italic',
                            letterSpacing: '0.5em',
                            color: 'rgba(0,0,0,0.5)',
                        }}
                    >
                        <div>documentation</div>
                    </div>
                </div>
                <Documentation />
            </Flex>
        </>
    );
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
        <Flex
            row
            css={css`
                display: flex;
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
        </Flex>
    );
}

function TopNavigation({ database }: { database: Database }): JSX.Element {
    const ghAPI = useGitHubAPI();
    useRenderOnEvent(database, 'dirty');

    return (
        <Flex
            row
            gap={6}
            css={css`
                padding: 4px 32px;
                border-bottom: 1px solid rgba(0, 0, 0, 0.8);
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            `}
        >
            <div>guidebook-bucket-list prototype</div>
            <Flex
                row
                gap={8}
                style={{
                    opacity: database.dirty ? 1 : 0,
                }}
                css={css`
                    color: #733;
                    transition: opacity 250ms;
                `}
            >
                <span style={{ fontStyle: 'italic' }}>unsaved changes</span>
            </Flex>
            <div style={{ flexGrow: 1 }} />
            {ghAPI ? <TopNavProfile ghAPI={ghAPI} /> : <div>login</div>}
        </Flex>
    );
}

function TopNavProfile({ ghAPI }: { ghAPI: GitHubAPI }) {
    const [user, setUser] = React.useState<any | null>(null);
    React.useEffect(() => {
        const go = async () => {
            setUser(await ghAPI?.user());
        };
        go();
    }, []);

    return (
        <Flex row gap={8}>
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
                logout
            </button>
        </Flex>
    );
}

function useRenderOnEvent(obj: any, event: string = 'modified') {
    const [_generation, setGeneration] = React.useState(1);
    React.useEffect(() => {
        return obj.events.on(event, () => {
            setGeneration((gen) => gen + 1);
        });
    }, [obj]);
}

function BucketListView({ database }: { database: Database }): JSX.Element {
    const [showDone, setShowDone] = React.useState(true);

    type SortItem = {
        field: keyof BucketItemData;
        reverse: boolean;
    };
    const [sortOrder, setSortOrder] = React.useState<SortItem[]>([
        { field: 'status', reverse: false },
        { field: 'year', reverse: false },
        { field: 'value', reverse: false },
        { field: 'name', reverse: false },
    ]);

    function pushOrder(s: keyof BucketItemData) {
        setSortOrder((order) => {
            if (order[0]?.field == s) {
                order[0].reverse = !order[0].reverse;
            } else {
                order = [{ field: s, reverse: false }, ...order.filter((o) => o.field != s)];
            }
            return [...order];
        });
    }

    // Wrapping the sorting in an effect without dependencies ensures
    // the items are sorted only once when the component is mounted.
    // This is good for UX as we don't want the item to "jump" to a new
    // position on a change to one of the values.
    const [items, setItems] = React.useState<BucketItem[]>(database.items);
    React.useEffect(() => {
        const items = [...database.items]
            .filter((item) => showDone || item.status !== 'done')
            .sort((a: BucketItem, b: BucketItem): number => {
                // "Hack" to put new items on the bottom of the list
                if (a.generation !== b.generation) {
                    if (a.generation === 1) {
                        return 1;
                    } else if (b.generation === 1) {
                        return -1;
                    }
                }

                for (const { field, reverse } of sortOrder) {
                    const r = reverse ? -1 : 1;
                    switch (field) {
                        case 'status':
                            if (a.status !== b.status) {
                                const table: { [k in BucketItemData['status']]: number } = {
                                    wip: 0,
                                    todo: 1,
                                    done: 2,
                                };
                                return r * (table[a.status] - table[b.status]);
                            }
                            break;
                        case 'category':
                            if (a.category !== b.category) {
                                return r * a.category.localeCompare(b.category);
                            }
                            break;
                        case 'value':
                            if (a.value !== b.value) {
                                return r * (b.value - a.value);
                            }
                            break;
                        case 'year':
                            if (a.year !== b.year) {
                                return r * (b.year - a.year);
                            }
                            break;
                        case 'name':
                            if (a.name !== b.name) {
                                return r * a.name.localeCompare(b.name);
                            }
                            break;
                        case 'rating':
                            if (a.rating !== b.rating) {
                                return r * (b.rating - a.rating);
                            }
                            break;
                        case 'description':
                            if (a.description !== b.description) {
                                return r * a.description.localeCompare(b.description);
                            }
                            break;
                        case 'review':
                            if (a.review !== b.review) {
                                return r * a.review.localeCompare(b.review);
                            }
                            break;
                        default:
                            console.warn('Unknown field:', field);
                    }
                }
                return 0;
            });

        setItems(items);
    }, [database.generation, sortOrder, showDone]);

    useRenderOnEvent(database);

    const ColumnHeader = ({ field }: { field: keyof BucketItemData }): JSX.Element => {
        return (
            <Flex
                css={css`
                    cursor: pointer;
                    user-select: none;
                    gap: 4px;
                `}
                onClick={() => pushOrder(field)}
            >
                <div>{field[0].toLocaleUpperCase() + field.slice(1)}</div>

                <div>{sortOrder[0]?.field === field && (sortOrder[0].reverse ? '^' : 'v')}</div>
            </Flex>
        );
    };

    return (
        <Flex col>
            <label>
                <input
                    type="checkbox"
                    checked={showDone}
                    onChange={() => {
                        setShowDone((v) => !v);
                    }}
                />
                Show done
            </label>

            <Flex
                col
                css={css`
                    .row {
                        > * {
                            flex: 0 0 120px;
                            padding: 4px 4px 4px 0px;
                        }

                        > :nth-child(1) {
                            flex: 0 0 12px;
                        }
                        > :nth-child(2) {
                            flex: 0 0 520px;
                        }
                        > :nth-child(3) {
                            flex: 0 0 110px;
                        }
                        > :nth-child(4) {
                            flex: 0 0 64px;
                        }
                        > :nth-child(5) {
                            flex: 0 0 90px;
                        }
                        > :nth-child(6) {
                            flex: 0 0 200px;
                        }
                        > :nth-child(7) {
                            flex: 0 0 60px;
                        }
                        > :nth-child(8) {
                            flex: 0 0 60px;
                        }
                        > :nth-child(9) {
                            flex: 0 0 60px;
                        }
                        > :nth-child(10) {
                            flex: 0 0 480px;
                        }

                        select {
                            color: inherit;
                            width: auto;
                            text-align: right;
                        }
                    }
                `}
            >
                <Flex
                    row
                    className="row"
                    style={{
                        fontWeight: 'bold',
                        backgroundColor: 'rgba(0,0,0,0.1)',
                        marginBottom: 2,
                        borderBottom: '2px solid rgba(0,0,0,0.1)',
                    }}
                >
                    <div />
                    <ColumnHeader field="name" />
                    <ColumnHeader field="category" />
                    <ColumnHeader field="status" />
                    <ColumnHeader field="value" />
                    <ColumnHeader field="description" />
                    <ColumnHeader field="year" />
                    <ColumnHeader field="month" />
                    <ColumnHeader field="rating" />
                    <ColumnHeader field="review" />
                </Flex>
                {items.map((item) => (
                    <BucketItemRow key={item.id} item={item} />
                ))}

                <Flex row>
                    <button
                        onClick={() => {
                            database.add({
                                name: 'Another new item',
                                status: 'todo',
                            });
                        }}
                    >
                        Add new
                    </button>
                </Flex>
            </Flex>
        </Flex>
    );
}

function idToColor(id: string): string {
    if (!id) {
        return '#777';
    }

    let value = 0;
    for (let i = 0; i < id.length; i++) {
        const charCode = id.charCodeAt(i);
        value = value * 256 + charCode;
    }
    const palette = [
        '#781e2e',
        '#a5bcbd',
        '#e7e3c7',
        '#f5b97b',
        '#ed8978',
        '#a45259',
        '#643159',
        '#816b24',
        '#96af2e',
        '#469852',
        '#9c2f8b',
        '#6950d1',
        '#7e94db',
        '#9bcea6',
        '#5bada6',
        '#127687',
        '#0a4684',
        '#181c38',
        '#5a4342',
        '#0e4a2c',
    ];
    return palette[value % palette.length];
}

function BucketItemRow({ item }: { item: BucketItem }): JSX.Element {
    useRenderOnEvent(item);

    const categories = item.database().itemCategories().sort();
    const years = item.database().itemYears().sort().reverse();

    return (
        <Flex
            key={item.id}
            className="row"
            row
            align="top"
            style={{
                backgroundColor:
                    item.status === 'done' ? '#4841' : item.status === 'wip' ? '#44F1' : 'inherit',
                opacity: item.status === 'done' ? 0.5 : 1,
            }}
            css={css`
                border-radius: 6px;
                color: ${idToColor(item.category)};

                select,
                option,
                input {
                    width: 100%;
                    color: inherit;
                    background-color: transparent;
                    font-size: inherit;
                    font-family: inherit;
                }
                select {
                    border: none;
                }
                input {
                    box-sizing: border-box;
                    border: solid 1px rgba(0, 0, 0, 0.01);
                    border-radius: 2px;
                }
            `}
        >
            <Flex row align="center">
                <div
                    style={{
                        width: 8,
                        minWidth: 8,
                        maxWidth: 8,
                        height: 8,
                        minHeight: 8,
                        maxHeight: 8,
                        margin: '0 0 2px 2px',
                        borderRadius: 4,
                        backgroundColor: idToColor(item.category),
                    }}
                />
            </Flex>
            <div>
                <input
                    type="text"
                    value={item.name}
                    style={{
                        color: idToColor(item.category),
                    }}
                    onChange={(evt) => {
                        item.modify({
                            name: evt.target.value,
                        });
                    }}
                />
            </div>

            <Flex
                row
                style={{
                    color: item.done() ? 'inherit' : idToColor(item.category),
                }}
            >
                <SelectWithNew item={item} field="category" values={categories} />
            </Flex>
            <Flex
                row
                align="start"
                style={{
                    cursor: 'pointer',
                }}
            >
                <select
                    value={item.status}
                    onChange={(evt) => {
                        item.modify({
                            status: evt.target.value as BucketItemData['status'],
                        });
                    }}
                >
                    <option value="todo">todo</option>
                    <option value="wip">wip</option>
                    <option value="done">done</option>
                </select>
            </Flex>
            <Flex row align="start">
                <div
                    style={{
                        paddingRight: 8,
                        width: '1.1ch',
                    }}
                >
                    {item.value === 0 ? '' : item.value}
                </div>
                <SmallButton
                    onClick={() => {
                        item.modify({
                            value: Math.min((item.value ?? 0) + 1, 10),
                        });
                    }}
                    onContextMenu={(evt: React.MouseEvent<HTMLDivElement>) => {
                        evt.preventDefault();
                        evt.stopPropagation();
                        item.modify({
                            value: Math.max((item.value ?? 0) - 1, 1),
                        });
                    }}
                >
                    ±
                </SmallButton>
            </Flex>
            <div
                style={{
                    fontSize: '0.8em',
                }}
            >
                {item.description}
            </div>
            <SelectWithNew
                item={item}
                field="year"
                values={years}
                transform={(s) => parseInt(s, 10) ?? 0}
            />
            <SelectWithNew
                item={item}
                disableNew={true}
                field="month"
                values={[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]}
                transform={(s) => parseInt(s, 10) ?? 0}
            />
            <SelectWithNew
                item={item}
                disableNew={true}
                field="rating"
                values={[1, 2, 3, 4, 5]}
                transform={(s) => parseInt(s, 10) ?? 0}
            />
            <div
                style={{
                    fontSize: '0.9em',
                    opacity: 0.8,
                    lineHeight: '1.1em',
                }}
            >
                {item.review}
            </div>
        </Flex>
    );
}

function SelectWithNew<T>({
    item,
    field,
    values,
    transform,
    disableNew,
}: {
    item: BucketItem;
    field: keyof BucketItemData;
    values: T[];
    transform?: (s: string) => T;
    disableNew?: boolean;
}): JSX.Element {
    const options = values.map((v) => `${v}`).filter((s) => !!s);
    const current = `${item.data[field]}` || '--';

    if (!options.includes(current)) {
        options.unshift(current);
    }

    return (
        <select
            value={current}
            onChange={(evt) => {
                let value = evt.target.value;
                if (value === '__new') {
                    value = (prompt('Enter new value:') ?? '').trim().toLocaleLowerCase();
                    if (!value) {
                        return;
                    }
                }

                item.modify({ [field]: transform ? transform(value) : value });
            }}
        >
            {!disableNew && <option value="__new">+ new</option>}
            {options.map((opt) => (
                <option key={opt} value={opt}>
                    {opt}
                </option>
            ))}
        </select>
    );
}

function SmallButton({
    onClick,
    onContextMenu,
    children,
}: {
    onClick?: () => void;
    onContextMenu?: (evt: React.MouseEvent<HTMLDivElement>) => void;
    children: React.ReactNode;
}): JSX.Element {
    const SIZE = 16;

    return (
        <div
            style={{
                display: 'flex',
                width: SIZE,
                height: SIZE,
                minWidth: SIZE,
                minHeight: SIZE,
                maxWidth: SIZE,
                maxHeight: SIZE,
                padding: '2px 2px',
                margin: '0 4px',
                border: '1px solid rgba(0,0,0,0.2)',
                borderRadius: 4,
                justifyContent: 'center',
                alignItems: 'center',

                userSelect: 'none',
                cursor: 'pointer',
            }}
            onClick={onClick}
            onContextMenu={onContextMenu}
        >
            {children}
        </div>
    );
}
