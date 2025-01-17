import React, { JSX } from 'react';
import { nanoid } from 'nanoid';
import { css } from './raiment-ui/use_css.tsx';
import { Flex } from './raiment-ui/flex.tsx';
import { Documentation } from './documentation.tsx';

type Callback = (...args: any[]) => any;
export class EventEmitter {
    _events: { [key: string]: Array<Callback> } = {};
    _queue: Array<[string, any[]]> = [];

    dispose() {
        this._events = {};
        this._queue = [];
    }

    on(event: string, callback: Callback): () => void {
        this._events[event] ??= [];
        this._events[event].push(callback);
        return () => {
            this.off(event, callback);
        };
    }

    once(event: string, callback: Callback) {
        const wrapper = (...args: any[]) => {
            callback(...args);
            this.off(event, wrapper);
        };
        this.on(event, wrapper);
    }

    off(event: string, callback: Callback) {
        if (this._events[event] === undefined) {
            throw new Error(`Cannot remove from unused event: '${event}'`);
        }
        this._events[event] = this._events[event].filter((cb) => cb !== callback);
    }

    fire(event: string, ...args: any[]) {
        const arr = this._events[event];
        if (arr && arr.length > 0) {
            return arr.map((cb) => cb(...args));
        }
        return [];
    }
}

type BucketItemData = {
    id: string;
    generation: number;
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

type BucketListData = {
    generation: number;
    items: BucketItemData[];
};

function replaceProperties<T>(data: Partial<T>, values: T): T {
    Object.keys(data).forEach((key: string) => {
        delete (data as any)[key];
    });
    Object.assign(data, values);
    return data as T;
}

class BucketItem {
    // ------------------------------------------------------------------------
    // Fields
    // ------------------------------------------------------------------------

    data: BucketItemData;
    events = new EventEmitter();
    _database: Database;

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    constructor(database: Database, data: Partial<BucketItemData>) {
        this._database = database;
        this.data = BucketItem.normalize(data);
    }

    static normalize(data: Partial<BucketItemData>): BucketItemData {
        return replaceProperties(data, {
            id: data.id ?? nanoid(16),
            generation: data.generation ?? 1,
            name: data.name || 'New item',
            category: data.category ?? '',
            status: data.status ?? 'todo',
            value: data.value ?? 0,
            year: data.year ?? 0,
            month: data.month ?? 0,
            rating: data.rating ?? 0,
            description: data.description ?? '',
            review: data.review ?? '',
        });
    }

    // ------------------------------------------------------------------------
    // Core properties
    // ------------------------------------------------------------------------

    database(): Database {
        return this._database;
    }

    get id(): string {
        return this.data.id;
    }
    get generation(): number {
        return this.data.generation;
    }
    get name(): string {
        return this.data.name;
    }
    get status(): BucketItemData['status'] {
        return this.data.status;
    }
    get category(): string {
        return this.data.category;
    }
    get value(): number {
        return this.data.value;
    }
    get description(): string {
        return this.data.description;
    }
    get year(): number {
        return this.data.year;
    }
    get month(): number {
        return this.data.month;
    }
    get rating(): number {
        return this.data.rating;
    }
    get review(): string {
        return this.data.review;
    }

    // ------------------------------------------------------------------------
    // Dervied / computer properties
    // ------------------------------------------------------------------------

    done(): boolean {
        return this.status === 'done';
    }

    // ------------------------------------------------------------------------
    // Mutation
    // ------------------------------------------------------------------------

    modify(data: Partial<BucketItemData>) {
        Object.assign(this.data, { ...data, generation: this.data.generation + 1 });
        this.events.fire('modified');
        this._database.save();
    }
}

class Database {
    data: BucketListData;
    events = new EventEmitter();
    _items: BucketItem[];

    constructor(data: Partial<BucketListData>) {
        this.data = Database.normalize(data);
        this._items = this.data.items.map((item) => new BucketItem(this, item));
    }

    get generation(): number {
        return this.data.generation;
    }

    get items(): BucketItem[] {
        return this._items;
    }

    static normalize(data: Partial<BucketListData>): BucketListData {
        const norm: BucketListData = {
            generation: data.generation ?? 1,
            items: data.items?.map((item) => BucketItem.normalize(item)) ?? [],
        };
        return norm;
    }

    static async load(): Promise<Database> {
        const response = await fetch('/api/read', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ path: 'bucket-list.yaml' }),
        });
        const data = await response.json();
        return new Database(data);
    }

    async save() {
        await fetch('/api/write', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                path: 'bucket-list.yaml',
                content: this.data,
            }),
        });
    }

    add(itemData: Partial<BucketItemData>) {
        const item = new BucketItem(this, itemData);
        this.data.items.push(item.data);
        this._items.push(item);
        this.data.generation += 1;
        this.events.fire('modified');
        this.save();
    }

    categories(): string[] {
        return Array.from(new Set(this.items.map((item) => item.category)));
    }

    itemYears(): number[] {
        return Array.from(new Set(this.items.map((item) => item.year)));
    }
}

export function App(): JSX.Element {
    const [database, setDatabase] = React.useState<Database | null>(null);

    React.useEffect(() => {
        const go = async () => {
            setDatabase(await Database.load());
        };
        go();
    }, []);

    if (!database) {
        return <p>Loading...</p>;
    }

    return (
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

function useUpdateOnModified(obj: any) {
    const [_generation, setGeneration] = React.useState(1);
    React.useEffect(() => {
        return obj.events.on('modified', () => {
            setGeneration((gen) => gen + 1);
        });
    }, [obj]);
}

function BucketListView({ database }: { database: Database }): JSX.Element {
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
            console.log(order);
            return [...order];
        });
    }

    // Wrapping the sorting in an effect without dependencies ensures
    // the items are sorted only once when the component is mounted.
    // This is good for UX as we don't want the item to "jump" to a new
    // position on a change to one of the values.
    const [items, setItems] = React.useState<BucketItem[]>(database.items);
    React.useEffect(() => {
        const items = [...database.items];
        items.sort((a: BucketItem, b: BucketItem): number => {
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
                }
            }
            return 0;
        });

        setItems(items);
    }, [database.generation, sortOrder]);

    useUpdateOnModified(database);

    return (
        <Flex
            col
            css={css`
                .row {
                    > * {
                        flex: 0 0 120px;
                        padding: 4px 4px 4px 0px;
                    }

                    > :nth-child(1) {
                        flex: 0 0 520px;
                    }
                    > :nth-child(2) {
                        flex: 0 0 110px;
                    }
                    > :nth-child(3) {
                        flex: 0 0 64px;
                    }
                    > :nth-child(4) {
                        flex: 0 0 90px;
                    }
                    > :nth-child(5) {
                        flex: 0 0 200px;
                    }
                    > :nth-child(6) {
                        flex: 0 0 60px;
                    }
                    > :nth-child(7) {
                        flex: 0 0 60px;
                    }
                    > :nth-child(8) {
                        flex: 0 0 60px;
                    }
                    > :nth-child(9) {
                        flex: 0 0 480px;
                    }

                    select {
                        color: inherit;
                        border: solid 1px rgba(0, 0, 0, 0.02);
                        border-radius: 2px;
                        width: auto;
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
                }}
            >
                <div onClick={() => pushOrder('name')}>Name</div>
                <div onClick={() => pushOrder('category')}>Category</div>
                <div onClick={() => pushOrder('status')}>Status</div>
                <div>Value</div>
                <div>Description</div>
                <div>Year</div>
                <div>Month</div>
                <div>Rating</div>
                <div>Review</div>
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
    );
}

function idToColor(id: string): string {
    let value = 0;
    for (let i = 0; i < id.length; i++) {
        const charCode = id.charCodeAt(i);
        value = value * 256 + charCode;
    }
    const palette = [
        '#979596',
        '#a5bcbd',
        '#e7e3c7',
        '#f5b97b',
        '#ed8978',
        '#a45259',
        '#643159',
        '#816b24',
        '#96af2e',
        '#469852',
        '#b967ad',
        '#6950d1',
        '#7e94db',
        '#9bcea6',
        '#5bada6',
        '#127687',
        '#0a4684',
        '#181c38',
        '#5a4342',
        '#686a69',
    ];
    return palette[value % palette.length];
}

function BucketItemRow({ item }: { item: BucketItem }): JSX.Element {
    useUpdateOnModified(item);

    const categories = item.database().categories().sort();
    const years = item.database().itemYears().sort().reverse();

    return (
        <Flex
            key={item.id}
            className="row"
            row
            align="top"
            style={{
                color: item.status === 'done' ? '#484' : item.status === 'wip' ? '#448' : 'inherit',
                opacity: item.status === 'done' ? 0.75 : 1,
            }}
            css={css`
                select,
                input {
                    width: 100%;
                    color: inherit;
                    font-size: inherit;
                    font-family: inherit;
                }
                input {
                    box-sizing: border-box;
                    border: solid 1px rgba(0, 0, 0, 0.1);
                    border-radius: 2px;
                }
            `}
        >
            <div>
                <input
                    type="text"
                    value={item.name}
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
                <div
                    style={{
                        width: 8,
                        minWidth: 8,
                        maxWidth: 8,
                        height: 8,
                        minHeight: 8,
                        maxHeight: 8,
                        borderRadius: 4,
                        backgroundColor: item.done() ? 'inherit' : idToColor(item.category),
                    }}
                />
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
                >
                    +
                </SmallButton>
                <SmallButton
                    onClick={() => {
                        item.modify({
                            value: Math.max((item.value ?? 2) - 1, 1),
                        });
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
                field="month"
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
