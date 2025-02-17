import { RNG } from '../raiment-core/index.ts';

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

function context2d(canvas: HTMLCanvasElement): CanvasRenderingContext2D {
    // Note that we *always* should call this helper with the wilLReadFrequently option
    // because for that option to ever take effect, it must be set on the first call
    // to getContext('2d') for that canvas.  Since this code is optimized to "just work",
    // always call it with this setting.
    const ctx = canvas.getContext('2d', {
        willReadFrequently: true,
    });
    if (!ctx) {
        throw new Error('Canvas context is not supported');
    }
    return ctx as CanvasRenderingContext2D;
}

/**
 * ImageMutator is a quick-and-dirty DOM-based class to modify an image.
 * It is designed for use in prototyping, therefore is biased toward
 * simple implementations with "good enough" performance.
 */
export class ImageMutator {
    _url: string;
    _commands: any[] = [];

    constructor(url: string) {
        this._url = url;
    }

    autocrop(): ImageMutator {
        this._commands.push({ type: 'autocrop' });
        return this;
    }

    rotate(deg: number): ImageMutator {
        this._commands.push({ type: 'rotate', deg });
        return this;
    }

    colorize(color: string): ImageMutator {
        this._commands.push({ type: 'colorize', color });
        return this;
    }

    resize(width: number, height: number): ImageMutator {
        this._commands.push({ type: 'resize', width, height });
        return this;
    }

    clampAlpha(): ImageMutator {
        this._commands.push({ type: 'clamp_alpha' });
        return this;
    }

    speckleColor(): ImageMutator {
        this._commands.push({ type: 'speckle_color' });
        return this;
    }

    blur(iterations: number): ImageMutator {
        this._commands.push({ type: 'blur', iterations });
        return this;
    }

    async toDataURL(): Promise<string> {
        let canvas = await this._load(this._url);
        while (this._commands.length > 0) {
            const cmd = this._commands.shift();
            canvas = this._runCommand(canvas, cmd);
        }
        return canvas.toDataURL();
    }

    _runCommand(canvas: HTMLCanvasElement, cmd: any): HTMLCanvasElement {
        switch (cmd.type) {
            case 'rotate':
                return this._rotate(canvas, cmd.deg);
            case 'colorize':
                return this._colorize(canvas, cmd.color);
            case 'autocrop':
                return this._autocrop(canvas);
            case 'resize':
                return this._resize(canvas, cmd.width, cmd.height);
            case 'clamp_alpha':
                return this._clampAlpha(canvas);
            case 'speckle_color':
                return this._speckleColor(canvas);
            case 'blur':
                return this._blur(canvas, cmd.iterations);
            default:
                throw new Error(`Unknown command type: ${cmd.type}`);
        }
    }

    _blur(canvas: HTMLCanvasElement, iterations: number): HTMLCanvasElement {
        for (let i = 0; i < iterations; i++) {
            canvas = this._blurOnce(canvas);
        }
        return canvas;
    }

    _blurOnce(canvas: HTMLCanvasElement): HTMLCanvasElement {
        const ctx = context2d(canvas);
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;

        const kernel = [
            [1, 2, 1],
            [2, 4, 2],
            [1, 2, 1],
        ];

        const kernelSize = 3;
        const kernelRadius = 1;

        const width = canvas.width;
        const height = canvas.height;

        const output = ctx.createImageData(width, height);
        const outputData = output.data;

        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                const i = (y * width + x) * 4;
                let r = 0;
                let g = 0;
                let b = 0;
                let a = 0;
                let t = 0;

                for (let ky = 0; ky < kernelSize; ky++) {
                    for (let kx = 0; kx < kernelSize; kx++) {
                        const k = kernel[ky][kx];
                        const dx = x + kx - kernelRadius;
                        const dy = y + ky - kernelRadius;
                        if (dx >= 0 && dx < width && dy >= 0 && dy < height) {
                            const j = (dy * width + dx) * 4;
                            r += data[j] * k;
                            g += data[j + 1] * k;
                            b += data[j + 2] * k;
                            a += data[j + 3] * k;
                            t += k;
                        }
                    }
                }

                outputData[i] = r / t;
                outputData[i + 1] = g / t;
                outputData[i + 2] = b / t;
                outputData[i + 3] = a / t;
            }
        }

        ctx.putImageData(output, 0, 0);

        return canvas;
    }

    _speckleColor(canvas: HTMLCanvasElement): HTMLCanvasElement {
        const ctx = context2d(canvas);

        const rng = RNG.make_random();
        const shades = [1.0, 1.0, 1.0, 0.95, 0.925, 0.9, 0.85];

        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;
        for (let i = 0; i < data.length; i += 4) {
            if (data[i + 3] !== 0) {
                const shade = rng.select(shades);
                data[i + 0] = Math.floor(data[i + 0] * shade);
                data[i + 1] = Math.floor(data[i + 1] * shade);
                data[i + 2] = Math.floor(data[i + 2] * shade);
            }
        }
        ctx.putImageData(imageData, 0, 0);
        return canvas;
    }

    _clampAlpha(canvas: HTMLCanvasElement): HTMLCanvasElement {
        const ctx = context2d(canvas);

        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;
        for (let i = 3; i < data.length; i += 4) {
            data[i] = data[i] > 0 ? 255 : 0;
        }
        ctx.putImageData(imageData, 0, 0);
        return canvas;
    }

    _resize(canvas: HTMLCanvasElement, width: number, height: number): HTMLCanvasElement {
        const aspectRatio = canvas.width / canvas.height;
        if (width / height > aspectRatio) {
            width = Math.ceil(height * aspectRatio);
        } else {
            height = Math.ceil(width / aspectRatio);
        }

        const target = document.createElement('canvas');
        const ctx = context2d(target);

        target.width = width;
        target.height = height;
        ctx.drawImage(canvas, 5, 5, width - 10, height - 10);

        return target;
    }

    _load(url: string): Promise<HTMLCanvasElement> {
        return new Promise((resolve, reject) => {
            const image = new Image();
            image.crossOrigin = 'anonymous';
            image.onload = () => {
                // Convert image to a canvas
                const canvas = document.createElement('canvas');
                const ctx = context2d(canvas);
                canvas.width = image.width;
                canvas.height = image.height;
                ctx.drawImage(image, 0, 0);
                resolve(canvas);
            };
            image.onerror = (err) => reject(err);
            image.src = url;
        });
    }

    _autocrop(canvas: HTMLCanvasElement): HTMLCanvasElement {
        const ctx = context2d(canvas);

        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;

        let top = 0;
        let left = 0;
        let right = canvas.width;
        let bottom = canvas.height;

        // Find top
        outer: for (let y = 0; y < canvas.height; y++) {
            for (let x = 0; x < canvas.width; x++) {
                const i = (y * canvas.width + x) * 4;
                if (data[i + 3] !== 0) {
                    top = y;
                    break outer;
                }
            }
        }

        // Find bottom
        outer: for (let y = canvas.height - 1; y >= 0; y--) {
            for (let x = 0; x < canvas.width; x++) {
                const i = (y * canvas.width + x) * 4;
                if (data[i + 3] !== 0) {
                    bottom = y + 1;
                    break outer;
                }
            }
        }

        // Find left
        outer: for (let x = 0; x < canvas.width; x++) {
            for (let y = 0; y < canvas.height; y++) {
                const i = (y * canvas.width + x) * 4;
                if (data[i + 3] !== 0) {
                    left = x;
                    break outer;
                }
            }
        }

        // Find right
        outer: for (let x = canvas.width - 1; x >= 0; x--) {
            for (let y = 0; y < canvas.height; y++) {
                const i = (y * canvas.width + x) * 4;
                if (data[i + 3] !== 0) {
                    right = x + 1;
                    break outer;
                }
            }
        }

        const width = right - left;
        const height = bottom - top;

        const cropped = document.createElement('canvas');
        cropped.width = width;
        cropped.height = height;
        context2d(cropped).drawImage(canvas, left, top, width, height, 0, 0, width, height);
        return cropped;
    }

    _colorize(canvas: HTMLCanvasElement, color: string): HTMLCanvasElement {
        const ctx = context2d(canvas);

        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;
        const rgb = hexToRgb(color);

        for (let i = 0; i < data.length; i += 4) {
            if (data[i + 3] !== 0) {
                data[i] = rgb[0];
                data[i + 1] = rgb[1];
                data[i + 2] = rgb[2];
                data[i + 3] = 255;
            }
        }

        ctx.putImageData(imageData, 0, 0);
        return canvas;
    }

    _rotate(source: HTMLCanvasElement, deg: number): HTMLCanvasElement {
        const target = document.createElement('canvas');
        const ctx = context2d(target);

        const angle = deg * (Math.PI / 180);
        const sin = Math.abs(Math.sin(angle));
        const cos = Math.abs(Math.cos(angle));

        target.width = 2 * (source.width * cos + source.height * sin);
        target.height = 2 * (source.width * sin + source.height * cos);

        ctx.translate(target.width / 2, target.height / 2);
        ctx.rotate(angle);
        ctx.drawImage(source, -source.width / 2, -source.height / 2);

        return target;
    }
}
