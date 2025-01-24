import React, { JSX } from 'react';
import { RNG } from '../snowfall-core/index.ts';
import { Div, Flex, css } from '../snowfall-ui/index.ts';

export function TownView({ seed }: { seed: number }): JSX.Element {
    const inst = React.useMemo(() => generate(seed), [seed]);

    return (
        <Flex row gap={32}>
            <Card inst={inst} />
            <PolygonCanvas poly={inst.shape} points={inst.locationPoints} />
        </Flex>
    );
}

function PolygonCanvas({ poly, points }: { poly: Polygon; points: Point[] }): JSX.Element {
    const ref = React.useRef<HTMLCanvasElement>(null);

    React.useEffect(() => {
        if (!poly) {
            return;
        }

        const ctx = ref.current!.getContext('2d');
        if (!ctx) {
            return;
        }
        ctx.clearRect(0, 0, 320, 320);

        ctx.strokeStyle = 'rgba(120, 80, 0, 0.5)';
        ctx.beginPath();
        ctx.moveTo(poly[0].x + 160, poly[0].y + 160);
        for (let i = 1; i < poly.length; i++) {
            ctx.lineTo(poly[i].x + 160, poly[i].y + 160);
        }
        ctx.closePath();
        ctx.stroke();

        ctx.fillStyle = 'rgba(55, 100, 200, 0.85)';
        for (const point of points) {
            ctx.beginPath();
            ctx.arc(point.x + 160, point.y + 160, 2.5, 0, 2 * Math.PI);
            ctx.fill();
        }
    }, [poly]);

    return (
        <canvas ref={ref} width={320} height={320} style={{ border: '1px solid black' }}></canvas>
    );
}

function Card({ inst }: { inst: Town }): JSX.Element {
    return (
        <Div
            css={css`
                display: flex;
                flex-direction: column;
                box-sizing: border-box;
                width: 300px;
                height: 500px;
                padding: 4px;
                border-radius: 8px;
                background-color: #302431;
                box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.5);

                .topbar {
                    display: flex;
                    justify-content: space-between;
                    padding: 4px;
                    border-bottom: 1px solid #666;
                    background-color: rgba(255, 255, 255, 0.65);
                    border: solid 1px #111;
                    border-radius: 8px;
                    padding: 0 6px;
                    box-shadow: 0px 0px 4px rgba(10, 128, 255, 0.95);

                    .name {
                        font-weight: bold;
                    }
                    .type {
                        font-weight: 100;
                        font-style: italic;
                        font-size: 80%;
                    }
                }
                .body {
                    flex-grow: 1;
                    margin: 4px 0 4px 0;
                    padding: 4px;
                    border-radius: 4px;
                    background-color: rgba(255, 255, 255, 0.7);
                }
                .bottombar {
                    display: flex;
                    justify-content: space-between;
                    padding: 4px;
                    border-bottom: 1px solid #666;
                    background-color: rgba(255, 255, 255, 0.25);
                    border: solid 1px #111;
                    border-radius: 8px;
                    font-size: 80%;
                    font-weight: 100;
                    padding: 0 4px;
                    margin: 0 0 4px 0;
                }
            `}
        >
            <Div cl="topbar">
                <Div cl="name">{inst.props.title}</Div>
                <Div cl="type">
                    {inst.name}/{inst.type[0].toUpperCase() + inst.type.slice(1)}
                </Div>
            </Div>
            <Div cl="body">
                <Table obj={inst} skip="id type title name seed props shape locationPoints" />
                <Table obj={inst.props} />
            </Div>
            <Div cl="bottombar">
                <Div></Div>
                <Div>
                    {inst.id} {inst.seed}
                </Div>
            </Div>
        </Div>
    );
}

function Table({ obj, skip }: { obj: Record<string, any>; skip?: string }): JSX.Element {
    const skipList = skip ? skip.split(' ').map((s) => s.trim()) : [];

    return (
        <Flex col>
            {Object.entries(obj)
                .filter(([key]) => {
                    return !skipList.includes(key);
                })
                .map(([key, value]) => (
                    <Flex key={key} row>
                        {typeof value !== 'object' ? (
                            <>
                                <Flex w="8em">{key}</Flex>
                                <Flex>{`${value}`}</Flex>
                            </>
                        ) : (
                            <>
                                <Flex w="1em"></Flex>
                                <Flex col>
                                    {Object.entries(value).map(([key, value]) => (
                                        <Flex key={key} row>
                                            <Flex w="8em">{key}</Flex>
                                            <Flex>{`${value}`}</Flex>
                                        </Flex>
                                    ))}
                                </Flex>
                            </>
                        )}
                    </Flex>
                ))}
        </Flex>
    );
}

type Point = { x: number; y: number };
type Polygon = Point[];

type Town = Record<string, any>;

function generateShape(rng: RNG): Polygon {
    const sides = rng.rangei(6, 12);
    const radius = 160;
    const angleIncrement = (2 * Math.PI) / sides;
    const points: Point[] = [];
    for (let i = 0; i < sides; i++) {
        const angle = i * angleIncrement + 0.25 * rng.range(0.0, Math.PI / sides);
        points.push({
            x: Math.cos(angle) * radius * rng.range(0.5, 1.0) + rng.range(-10, 10),
            y: Math.sin(angle) * radius * rng.range(0.5, 1.0) + rng.range(-10, 10),
        });
    }

    const scale = rng.range(0.35, 1.0);
    const rotation = rng.range(0, 2 * Math.PI);
    for (let i = 0; i < points.length; i++) {
        const x = points[i].x * scale;
        const y = points[i].y;
        points[i].x = x * Math.cos(rotation) - y * Math.sin(rotation);
        points[i].y = x * Math.sin(rotation) + y * Math.cos(rotation);
    }

    return points;
}

const isPointInPolygon = (point: Point, polygon: Point[]): boolean => {
    let inside = false;
    const n = polygon.length;

    for (let i = 0, j = n - 1; i < n; j = i++) {
        const xi = polygon[i].x;
        const yi = polygon[i].y;
        const xj = polygon[j].x;
        const yj = polygon[j].y;

        const intersect =
            yi > point.y !== yj > point.y &&
            point.x < ((xj - xi) * (point.y - yi)) / (yj - yi) + xi;

        if (intersect) inside = !inside;
    }

    return inside;
};

function generate(seed: number): Town {
    const rng = new RNG(seed);

    function sel<T>(t: string, s: string): string {
        const arr = s.split(' ').map((s) => s.trim());
        return t + rng.select(arr);
    }

    const nameFn = rng.select([
        () => sel('River', 'wood stone hill dale brook bend'),
        () => sel('Wood', 'ville town river burr'),
        () => 'Bramblewood',
        () => 'Nors',
        () => 'Mills',
        () => 'Bellfound',
        () => 'Shellborne',
        () => 'Hilltop',
        () => 'Dale',
        () => 'Burr',
        () => 'Stonehaul',
        () => 'Billows',
        () => 'Fenn',
        () => 'Barrow',
        () => sel('Barrow', 'fold glen stone'),
        () => 'Vale',
        () => 'Fell',
        () => sel('Plain', 'view town glen dale'),
        () => sel('Oak', 'sill tree field'),
        () => 'Rockshawl',
        () => sel('Bill', 'yard meadow'),
        () => sel('Middle', 'view vale town ton'),
        () => sel('Mid', 'ville creek glen ton'),
        () => sel('Black', 'rock bill ton'),
        () => sel('Yellow', 'oak tree field'),
        () => sel('Green', 'oak tree forest field burr'),
        () => sel('Hollow', 'vale fell plain'),
    ]);

    const props = {
        title: nameFn(),
        size: rng.select(['small', 'medium', 'large']),
        population: 2 + rng.d10(),
        locations: 2 + rng.d6(),
    };

    const shape = generateShape(rng);
    const locationPoints = [];

    const S = 0.9;
    const scaledShape = shape.map((point) => ({ x: point.x * S, y: point.y * S }));

    for (
        let attempts = 0;
        attempts < props.locations * 20 && locationPoints.length < props.locations;
        attempts++
    ) {
        const x = rng.range(-160, 160);
        const y = rng.range(-160, 160);
        if (!isPointInPolygon({ x, y }, scaledShape)) {
            continue;
        }
        let valid = true;
        for (let j = 0; j < locationPoints.length; j++) {
            const q = locationPoints[j];
            const dx = x - q.x;
            const dy = y - q.y;
            const dist = Math.sqrt(dx * dx + dy * dy);
            if (dist < 20) {
                valid = false;
                break;
            }
        }
        if (!valid) {
            continue;
        }

        locationPoints.push({ x, y });
    }

    return {
        name: 'Town',
        seed,
        id: 'town',
        type: 'area',
        props,
        shape,
        locationPoints,
    };
}
