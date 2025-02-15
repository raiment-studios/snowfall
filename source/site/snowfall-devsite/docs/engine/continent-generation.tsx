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

class DrawBuffer {
    buffer = new Array(4 * 1024 * 1024);
}

async function processRegions(deck: RegionCard[], startName: string): Promise<ImageData> {
    const start = deck.find((card) => card.data.name === startName)!;
    deck = deck.filter((card) => card !== start);

    const target = document.createElement('canvas');
    target.width = 1024;
    target.height = 1024;
    const targetCtx = target.getContext('2d')!;
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
        source.width = image.width;
        source.height = image.height;
        const context = source.getContext('2d')!;
        context.drawImage(image, 0, 0);
        const sourceData = context.getImageData(0, 0, image.width, image.height);

        let drawCount = 0;
        let pixelCount = 0;

        const backup = targetData.data.slice();
        const a = Math.PI / 2 + (angle * Math.PI) / 180;
        const ox = Math.round(cx + Math.cos(a) * distance) - Math.floor(image.width / 2);
        const oy = Math.round(cy + Math.sin(a) * distance) - Math.floor(image.height / 2);

        for (let y = 0; y < image.height; y++) {
            for (let x = 0; x < image.width; x++) {
                const i = 4 * (y * image.width + x);
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

                for (let k = 0; k < 4; k++) {
                    targetData.data[j + k] = sourceData.data[i + k];
                }
                drawCount += 1;
            }
        }

        if (drawCount < pixelCount * 0.8) {
            targetData.data.set(backup);
            console.log(
                card.data.name,
                drawCount,
                pixelCount,
                drawCount / pixelCount,
                angle,
                distance
            );
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
            console.log('adding', neighborName);
            queue.push({
                cx: ox + Math.floor(image.height / 2),
                cy: oy + Math.floor(image.width / 2),
                angle: angle + (Math.random() * 20 - 10),
                distance: 5 + Math.floor(Math.random() * 5),
                card: neighbor,
            });
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
            <canvas ref={ref} width={1024} height={1024} />
        </div>
    );
}
