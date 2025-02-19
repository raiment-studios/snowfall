import { RNG } from '../raiment-core/rng.ts';
import { RegionInstance } from '../world.ts';

function hexToRgb(hex: string): [number, number, number] {
    const match = hex.match(/^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i);
    return match
        ? [
              parseInt(match[1], 16), //
              parseInt(match[2], 16),
              parseInt(match[3], 16),
          ]
        : [0, 0, 0];
}

/**
 * For prototyping purposes, let's fix the map to 1024x1024 sectors.  We can
 * create dynamically sized map later.
 */

export class WorldMap {
    palette: (RegionInstance | null)[] = [];
    map: Uint16Array = new Uint16Array(1024 * 1024);
    width: number = 1024;
    height: number = 1024;

    constructor() {
        this.palette.push(null);
        this.palette.push({
            color: '#FF0000',
        } as RegionInstance);
        for (let i = 0; i < this.width * this.height; i++) {
            this.map[i] = 0;
        }
    }

    async place(region: RegionInstance, x: number, y: number, deg: number) {
        let cx = x + 512;
        let cy = y + 512;

        const image = new Image();
        image.src = region.bitmap;
        await new Promise((resolve) => {
            image.onload = resolve;
        });

        cx -= Math.floor(image.width / 2);
        cy -= Math.floor(image.height / 2);

        const index = this.palette.length;
        this.palette.push(region);

        // Get Image Data for image
        const canvas = document.createElement('canvas');
        canvas.width = image.width;
        canvas.height = image.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2d context');
        }
        ctx.drawImage(image, 0, 0);
        const imageData = ctx.getImageData(0, 0, image.width, image.height);
        const data = imageData.data;

        let placement = null;

        const original = this.map.slice();
        let dist = Math.floor(Math.min(image.width, image.height) / 3.0);
        let attempts = 1000;
        do {
            const ox = Math.floor(dist * Math.cos((-deg * Math.PI) / 180));
            const oy = Math.floor(dist * Math.sin((-deg * Math.PI) / 180));

            let total = 0;
            let skipped = 0;
            for (let ix = 0; ix < image.width; ix++) {
                for (let iy = 0; iy < image.height; iy++) {
                    const alpha = data[(ix + iy * image.width) * 4 + 3];
                    if (alpha === 0) {
                        continue;
                    }
                    total += 1;

                    const px = ox + cx + ix;
                    const py = oy + cy + iy;
                    const p = px + py * 1024;
                    if (this.map[p] !== 0) {
                        skipped += 1;
                        continue;
                    }
                    this.map[p] = index;
                }
            }

            if (skipped > total * 0.2) {
                dist += 3;
                this.map = original.slice();
            } else {
                placement = [
                    cx + ox - 512 + Math.floor(image.width / 2),
                    cy + oy - 512 + Math.floor(image.height / 2),
                ];
                break;
            }
            attempts -= 1;
        } while (attempts > 0);

        fillGaps2(this, index);
        fillGaps(this, index);

        return placement as [number, number];
    }

    toDataURL(): string {
        const rng = RNG.make_random();
        const shades = [1.0, 1.0, 0.98, 0.95, 0.92, 0.9];

        const canvas = document.createElement('canvas');
        canvas.width = 1024;
        canvas.height = 1024;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2d context');
        }

        const imageData = ctx.getImageData(0, 0, 1024, 1024);
        const data = imageData.data;

        for (let i = 0; i < 1024 * 1024; i++) {
            const region = this.palette[this.map[i]];
            if (region) {
                const rgb = hexToRgb(region.color);
                const s = rng.select(shades);
                const index = i * 4;
                data[index + 0] = Math.floor(s * rgb[0]);
                data[index + 1] = Math.floor(s * rgb[1]);
                data[index + 2] = Math.floor(s * rgb[2]);
                data[index + 3] = 255;
            } else {
                const index = i * 4;
                data[index + 0] = 0;
                data[index + 1] = 64;
                data[index + 2] = 128;
                data[index + 3] = 255;
            }
        }
        ctx.putImageData(imageData, 0, 0);
        return canvas.toDataURL();
    }
}

/**
 * Find "gaps" in the world map: i.e. cells that are empty and are
 * "landlocked" by other filled cells.
 *
 * This is *prototype* quality code and probably very inefficient.
 */
function fillGaps(worldMap: WorldMap, fillIndex: number) {
    // Build a mask marking marking the "boundary" cells - i.e. cells
    // that are next to the image edge OR are next to another boundary
    // cell. This will mean the non-boundary empty cells are implicitly
    // the gaps we're looking for.
    //
    const mask = new Uint8Array(worldMap.width * worldMap.height);
    mask.fill(0);

    // Initialize the algorithm by marking the top-left as a boundary,
    // which it always is given it is adjacent to the top and left edge.
    mask[0] = 1;

    // Process the queue of boundary cells, marking any empty neighbors
    // as also being boundary cells.
    const list = [[0, 0]];
    while (list.length > 0) {
        const [x, y] = list.pop()!;
        const i = x + y * worldMap.width;
        if (mask[i] !== 1) {
            continue;
        }

        for (let dy = -1; dy <= 1; dy += 1) {
            for (let dx = -1; dx <= 1; dx += 1) {
                if (
                    x + dx < 0 ||
                    x + dx > worldMap.width - 1 ||
                    y + dy < 0 ||
                    y + dy > worldMap.height - 1
                ) {
                    continue;
                }

                const j = i + dx + dy * worldMap.width;
                if (worldMap.map[j] === 0 && mask[j] === 0) {
                    mask[j] = 1;
                    list.push([x + dx, y + dy]);
                }
            }
        }
    }

    // The gaps are the non-boundary empty cells.  Fill them
    // with the given fill index.
    for (let x = 0; x < worldMap.width; x++) {
        for (let y = 0; y < worldMap.height; y++) {
            const i = x + y * worldMap.width;
            if (mask[i] === 0 && worldMap.map[i] === 0) {
                worldMap.map[i] = fillIndex;
            }
        }
    }
}

function fillGaps2(worldMap: WorldMap, fillIndex: number) {
    // For each vertical scanline, check if there's a gap of N or less between
    // one filled cell of fillIndex and the next.  If yes, fill it that gap with
    // fillIndex.

    for (let x = 0; x < worldMap.width; x++) {
        let y = 0;

        const get = (y: number) => worldMap.map[x + y * worldMap.width];

        while (y < worldMap.height && get(y) !== fillIndex) {
            y += 1;
        }

        let len = 0;
        while (y < worldMap.height && get(y) === fillIndex) {
            y += 1;
            len += 1;
        }

        // Count how many values are equal to 0
        let gap = 0;
        const gapStart = y;
        while (y < worldMap.height && get(y) === 0) {
            gap += 1;
            y += 1;
        }

        if (gap <= Math.min(len / 2, 32)) {
            for (let i = 0; i < gap; i++) {
                worldMap.map[x + (gapStart + i) * worldMap.width] = fillIndex;
            }
        }
    }
}
