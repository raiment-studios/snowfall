import { rgb24 } from 'jsr:@gnome/ansi@0.2';

export function cprintln(color?: string, msg?: string) {
    if (!color && !msg) {
        console.log();
        return;
    }
    if (msg == undefined) {
        msg = color;
        color = '#779';
    }

    let s = rgb24(msg!, parseColor(color!));
    console.log(s);
}

export function rgb(color: string, msg: string) {
    return rgb24(msg, parseColor(color));
}

export function parseColor(color: string): { r: number; g: number; b: number } {
    // Named colors
    color =
        {
            error: '#f00',
            warn: '#d90',
            key: '#4CF',
        }[color] ?? color;

    // Convert 6 or 3 digit hex to RGB
    color = color.trim().replace(/^#/, '');
    if (color.length === 6) {
        const r = parseInt(color.slice(0, 2), 16);
        const g = parseInt(color.slice(2, 4), 16);
        const b = parseInt(color.slice(4, 6), 16);
        return { r, g, b };
    } else if (color.length === 3) {
        const r = parseInt(color[0] + color[0], 16);
        const g = parseInt(color[1] + color[1], 16);
        const b = parseInt(color[2] + color[2], 16);
        return { r, g, b };
    } else {
        return { r: 255, g: 200, b: 100 };
    }
}
