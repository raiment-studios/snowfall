import { nanoid } from 'nanoid';
import { EventEmitter } from './guidebook-ui/event_emitter.ts';
import { GitHubAPI } from './guidebook-ui/use_github_auth.ts';
import yaml from 'js-yaml';

export type BucketItemData = {
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

export type BucketListData = {
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

export class BucketItem {
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

export class Database {
    data: BucketListData;
    events = new EventEmitter();
    ghAPI: GitHubAPI;
    _items: BucketItem[];

    constructor(data: Partial<BucketListData>, ghAPI: GitHubAPI) {
        this.ghAPI = ghAPI;
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

    static async load(ghAPI: GitHubAPI): Promise<Database> {
        const encoded = await ghAPI.readFileContents('guidebook/bucket-list/bucket-list.yaml');
        const data = yaml.load(encoded);
        return new Database(data, ghAPI);
    }

    dirty = false;

    _saveTimeout: number | null = null;
    save(): Promise<void> {
        this.dirty = true;
        this.events.fire('dirty');
        return new Promise((resolve) => {
            if (this._saveTimeout !== null) {
                clearTimeout(this._saveTimeout);
            }

            const contentJSON = JSON.stringify(this.data);
            const contentYAML = yaml.dump(this.data);
            localStorage.setItem('bucket-list', contentJSON);

            this._saveTimeout = setTimeout(async () => {
                console.log('Saving GitHub data...');
                await this.ghAPI.updateFileContents(
                    'guidebook/bucket-list/bucket-list.yaml',
                    contentYAML
                );
                this.dirty = false;
                this.events.fire('dirty');
                resolve();
            }, 2500);
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

    itemCategories(): string[] {
        return Array.from(new Set(this.items.map((item) => item.category)));
    }

    itemYears(): number[] {
        return Array.from(new Set(this.items.map((item) => item.year)));
    }

    select({ id }: { id?: string | null }): BucketItem | null {
        if (id !== undefined) {
            return this.items.find((item) => item.id === id) ?? null;
        }
        return null;
    }
}
