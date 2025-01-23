import { randomSeeded, Prng } from '@std/random';

export class RNG {
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
    d8k(): number {
        return 1 + this.rangei(0, 8192);
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

    shuffle<T>(arr: T[]): T[] {
        const copy = arr.slice();
        for (let i = copy.length - 1; i > 0; i--) {
            const j = this.rangei(0, i + 1);
            [copy[i], copy[j]] = [copy[j], copy[i]];
        }
        return copy;
    }
}
