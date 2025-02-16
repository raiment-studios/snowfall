import React from 'react';

type NeighborData = [string, number];

type RegionCardData = {
    name: string;
    url: string;
    neighbors: NeighborData[];
};

class RegionCard {
    data: RegionCardData;
    constructor(data: RegionCardData) {
        this.data = data;
    }
}

const regionCardData: RegionCardData[] = [
    {
        name: 'region-0',
        url: '/content/region-bitmap-00.png',
        neighbors: [
            ['region-1', 90], //
            ['region-2', 30], //
            ['region-3', -20], //
        ],
    },
    {
        name: 'region-1',
        url: '/content/region-bitmap-01.png',
        neighbors: [
            ['region-5', 110], //
        ],
    },
    {
        name: 'region-2',
        url: '/content/region-bitmap-02.png',
        neighbors: [
            ['region-4', 90], //
        ],
    },
    {
        name: 'region-3',
        url: '/content/region-bitmap-03.png',
        neighbors: [],
    },
    {
        name: 'region-4',
        url: '/content/region-bitmap-04.png',
        neighbors: [],
    },
    {
        name: 'region-5',
        url: '/content/region-bitmap-05.png',
        neighbors: [],
    },
];

const regionCards = regionCardData.map((data) => new RegionCard(data));

async function processRegions(deck: RegionCard[], startName: string): Promise<ImageData> {
    const start = deck.find((card) => card.data.name === startName)!;
    deck = deck.filter((card) => card !== start);

    const target = document.createElement('canvas');
    target.style.imageRendering = 'pixelated';
    target.width = 1000;
    target.height = 1000;
    const targetCtx = target.getContext('2d')!;
    targetCtx.imageSmoothingEnabled = false;
    let targetData = targetCtx.getImageData(0, 0, target.width, target.height);

    const queue = [
        {
            cx: 512,
            cy: 512,
            distance: 0,
            angle: 0,
            card: start,
        },
    ];

    while (queue.length > 0) {
        const entry = queue.shift()!;
        const { cx, cy, distance, angle, card } = entry;

        // Load the active card url into an Image
        const image = new Image();
        image.src = card.data.url;
        await new Promise((resolve) => {
            image.onload = resolve;
        });

        // Get the RGBA data of the image
        const source = document.createElement('canvas');
        source.width = image.width * 2;
        source.height = image.height * 2;
        source.style.imageRendering = 'pixelated';
        const context = source.getContext('2d')!;
        context.imageSmoothingEnabled = false;
        context.save();
        context.translate(source.width / 2, source.height / 2);
        context.rotate(((2 * Math.random() - 1) * Math.PI) / 16);
        context.scale(0.1 * (Math.random() * 2 - 1) + 1.0, 0.1 * (Math.random() * 2 - 1) + 1.0);
        context.drawImage(
            image,
            -source.width / 2,
            -source.height / 2,
            source.width,
            source.height
        );
        context.restore();
        const sourceData = context.getImageData(0, 0, source.width, source.height);
        for (let y = 0; y < source.height; y++) {
            for (let x = 0; x < source.width; x++) {
                const i = 4 * (y * source.width + x);
                if (sourceData.data[i + 3] !== 0) {
                    sourceData.data[i + 3] = 255;
                }
            }
        }

        let drawCount = 0;
        let pixelCount = 0;

        const backup = targetData.data.slice();
        const a = Math.PI / 2 + (angle * Math.PI) / 180;
        const ox = Math.round(cx + Math.cos(a) * distance) - Math.floor(source.width / 2);
        const oy = Math.round(cy + Math.sin(a) * distance) - Math.floor(source.height / 2);

        for (let y = 0; y < source.height; y++) {
            for (let x = 0; x < source.width; x++) {
                const i = 4 * (y * source.width + x);
                const j = 4 * ((ox + y) * target.width + (oy + x));

                const sourceAlpha = sourceData.data[i + 3];
                if (sourceAlpha === 0) {
                    continue;
                }
                pixelCount += 1;

                const destAlpha = targetData.data[j + 3];
                if (destAlpha !== 0) {
                    continue;
                }

                targetData.data[j + 0] = sourceData.data[i + 0];
                targetData.data[j + 1] = sourceData.data[i + 1];
                targetData.data[j + 2] = sourceData.data[i + 2];
                targetData.data[j + 3] = 255;

                drawCount += 1;
            }
        }

        if (drawCount < pixelCount * 0.8) {
            targetData.data.set(backup);
            queue.unshift({
                ...entry,
                distance: distance + Math.floor(Math.random() * 5 + 2),
            });
            await new Promise((resolve) => setTimeout(resolve, 5));
            continue;
        }

        for (const [neighborName, angle] of card.data.neighbors) {
            const neighbor = deck.find((card) => card.data.name === neighborName)!;
            if (!neighbor) {
                continue;
            }
            deck = deck.filter((card) => card !== neighbor);
            queue.push({
                cx: ox + Math.floor(image.height / 2),
                cy: oy + Math.floor(image.width / 2),
                angle: angle + (Math.random() * 20 - 10),
                distance: 5 + Math.floor(Math.random() * 5),
                card: neighbor,
            });
        }
    }

    // Fill gaps in the regions
    if (true) {
        const check = (x: number, y: number) => {
            const i = 4 * (y * target.width + x);
            if (targetData.data[i + 3] === 0) {
                targetData.data[i + 0] = 255;
                targetData.data[i + 1] = 0;
                targetData.data[i + 2] = 0;
                targetData.data[i + 3] = 128;
                return true;
            } else {
                return false;
            }
        };

        const get = (x: number, y: number) => {
            const i = 4 * (y * target.width + x);
            return targetData.data.slice(i, i + 4);
        };
        const set = (x: number, y: number, rgba: number[]) => {
            const i = 4 * (y * target.width + x);
            targetData.data.set(rgba, i);
        };

        for (let y = 0; y < target.height; y++) {
            for (let x = 0; x < target.width; x++) {
                if (!check(x, y)) {
                    break;
                }
            }
            for (let x = target.width - 1; x >= 0; x--) {
                if (!check(x, y)) {
                    break;
                }
            }
        }

        for (let x = 0; x < target.width; x++) {
            for (let y = 0; y < target.height; y++) {
                if (!check(x, y)) {
                    break;
                }
            }
            for (let y = target.height - 1; y >= 0; y--) {
                if (!check(x, y)) {
                    break;
                }
            }
        }

        // This multi-pass gap filling is a lot slower, but avoids the
        // hard, straight line fills of simply filling the gaps via x/y
        // scans.
        const gaps: [number, number, any][] = [];
        while (true) {
            for (let y = 0; y < target.height; y++) {
                for (let x = 0; x < target.width; x++) {
                    const rgba = get(x, y);
                    if (rgba[3] !== 0) {
                        continue;
                    }

                    const options = [];
                    const left = get(x - 1, y);
                    const right = get(x + 1, y);
                    const up = get(x, y - 1);
                    const down = get(x, y + 1);
                    if (left[3] !== 0) {
                        options.push(left);
                    } else if (right[3] !== 0) {
                        options.push(right);
                    } else if (up[3] !== 0) {
                        options.push(up);
                    } else if (down[3] !== 0) {
                        options.push(down);
                    }
                    if (options.length > 0) {
                        gaps.push([x, y, options]);
                    }
                }
            }
            if (gaps.length === 0) {
                break;
            }
            while (gaps.length > 0) {
                const j = Math.floor(Math.random() * gaps.length);
                const [x, y, options] = gaps[j];
                gaps.splice(j, 1);

                const k = Math.floor(Math.random() * options.length);
                const rgba = options[k];
                set(x, y, rgba);
            }
        }

        for (let y = 0; y < target.height; y++) {
            for (let x = 0; x < target.width; x++) {
                const rgba = get(x, y);
                if (rgba[3] === 128) {
                    set(x, y, [0, 0, 0, 0]);
                }
            }
        }
    }

    return targetData;
}

export function Demo() {
    const ref = React.useRef<HTMLCanvasElement>(null);
    React.useEffect(() => {
        const go = async () => {
            if (!ref.current) {
                return;
            }
            const canvas = ref.current!;
            const imageData = await processRegions(regionCards, 'region-0');

            canvas.width = imageData.width;
            canvas.height = imageData.height;

            // Draw the imageData to the canvas
            const ctx = canvas.getContext('2d')!;
            ctx.putImageData(imageData, 0, 0);
        };
        go();
    }, [ref.current]);

    return (
        <div style={{ border: 'solid 1px #555' }}>
            <canvas ref={ref} width={100} height={1000} />
        </div>
    );
}
