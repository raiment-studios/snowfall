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
