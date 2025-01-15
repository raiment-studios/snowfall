import React, { JSX } from 'react';
import { nanoid } from 'nanoid';

import { Documentation } from './documentation.tsx';

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
    onClick,
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

    onClick?: (evt: React.MouseEvent<HTMLDivElement>) => void;

    children?: React.ReactNode;
}): JSX.Element {
    const computed: React.CSSProperties = {
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
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
        <div className={className} style={computed} onClick={onClick}>
            {children}
        </div>
    );
}

type BucketItem = {
    id: string;
    name: string;
    category: string;
    status: 'todo' | 'wip' | 'done';
    value: number;
    year: number;
    month: number;
    rating: number;
    description: string;
    review: string;
};

type BucketList = {
    items: BucketItem[];
};

type Commands = {
    save(doc: BucketList): void;
};

export function App(): JSX.Element {
    const [doc, setDoc] = React.useState<BucketList | null>(null);

    React.useEffect(() => {
        const go = async () => {
            const response = await fetch('/api/read', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ path: 'bucket-list.yaml' }),
            });
            const data = await response.json();
            normalize(data);
            setDoc(data);
        };
        go();
    }, []);

    if (!doc) {
        return <p>Loading...</p>;
    }

    const commands: Commands = {
        save(doc) {
            const go = async () => {
                setDoc({ ...doc });
                fetch('/api/write', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        path: 'bucket-list.yaml',
                        content: doc,
                    }),
                });
            };
            go();
        },
    };

    return (
        <Flex col m="12px 64px">
            <Flex row>
                <h1>Bucket List</h1>
                <div style={{ width: 32 }} />
                <button onClick={() => commands.save(doc)}>save</button>
            </Flex>
            <BucketListView commands={commands} doc={doc} />

            <div>
                <div
                    style={{
                        margin: '128px 0 32px',
                        borderTop: '1px solid rgba(0,0,0,0.1)',
                        paddingLeft: 64,
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
    );
}

function normalize(doc: BucketList) {
    doc.items = doc.items.map((item) => {
        return {
            id: item.id ?? nanoid(16),
            name: item.name ?? '',
            category: item.category ?? '',
            status: item.status ?? 'todo',
            value: item.value ?? 0,
            year: item.year ?? 0,
            month: item.month ?? 0,
            rating: item.rating ?? 0,
            description: item.description ?? '',
            review: item.review ?? '',
        };
    });
}

function BucketListView({ doc, commands }: { doc: BucketList; commands: Commands }): JSX.Element {
    // Wrapping the sorting in an effect without dependencies ensures
    // the items are sorted only once when the component is mounted.
    // This is good for UX as we don't want the item to "jump" to a new
    // position on a change to one of the values.
    const [items, setItems] = React.useState<BucketItem[]>(doc.items);
    React.useEffect(() => {
        const items = [...doc.items].sort((a: BucketItem, b: BucketItem): number => {
            if (a.status !== b.status) {
                const table: { [k in BucketItem['status']]: number } = {
                    wip: 0,
                    todo: 1,
                    done: 2,
                };
                return table[a.status] - table[b.status];
            }
            return a.name.localeCompare(b.name);
        });

        setItems(items);
    }, []);

    return (
        <div>
            <style>
                {`
                .row {
                    > * {
                        flex : 1 0 0;
                        padding: 4px 2px;
                    }

                    > :nth-child(1) {
                        flex : 4 0 20ch;
                        
                    }
                    > :nth-child(2) {
                        flex: 1 0 0;
                    }
                    > :nth-child(3) {
                        flex: 1 0 4ch;
                    }
                    > :nth-child(9) {
                        flex : 2 0 12ch;
                    }
                }
            `}
            </style>
            <Flex col>
                <Flex
                    row
                    className="row"
                    style={{
                        fontWeight: 'bold',
                        backgroundColor: 'rgba(0,0,0,0.1)',
                    }}
                >
                    <div>Name</div>
                    <div>Category</div>
                    <div>Status</div>
                    <div>Value</div>
                    <div>Description</div>
                    <div>Year</div>
                    <div>Month</div>
                    <div>Rating</div>
                    <div>Review</div>
                </Flex>
                {items.map((item) => (
                    <Flex
                        key={item.id}
                        className="row"
                        row
                        align="top"
                        style={{
                            color:
                                item.status === 'done'
                                    ? '#484'
                                    : item.status === 'wip'
                                    ? '#448'
                                    : 'inherit',
                            opacity: item.status === 'done' ? 0.75 : 1,
                        }}
                    >
                        <div>{item.name}</div>
                        <div>{item.category}</div>
                        <Flex
                            row
                            align="start"
                            style={{
                                cursor: 'pointer',
                            }}
                            onClick={(evt: React.MouseEvent<HTMLDivElement>) => {
                                evt.preventDefault();
                                evt.stopPropagation();

                                type Status = BucketItem['status'];
                                const table: { [key in Status]: Status } = {
                                    todo: 'wip',
                                    wip: 'done',
                                    done: 'todo',
                                };
                                item.status = table[item.status] ?? 'todo';
                                commands.save(doc);
                            }}
                        >
                            {item.status === 'todo' ? '' : item.status}
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
                                    item.value ??= 0;
                                    item.value = Math.min(item.value + 1, 10);
                                    commands.save(doc);
                                }}
                            >
                                +
                            </SmallButton>
                            <SmallButton
                                onClick={() => {
                                    item.value ??= 0;
                                    item.value = Math.max(item.value - 1, 0);
                                    commands.save(doc);
                                }}
                            >
                                -
                            </SmallButton>
                        </Flex>
                        <div
                            style={{
                                fontSize: '0.8em',
                            }}
                        >
                            {item.description}
                        </div>
                        <div>{item.year === 0 ? '' : item.year}</div>
                        <div>{item.month == 0 ? '' : item.month}</div>
                        <div>{item.rating == 0 ? '' : item.rating}</div>
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
                ))}
            </Flex>
        </div>
    );
}

function SmallButton({
    onClick,
    children,
}: {
    onClick?: () => void;
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
        >
            {children}
        </div>
    );
}
