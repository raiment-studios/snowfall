import { cprintln, rgb } from '../cprint.ts';

export function command_cprintln(args: string[]) {
    const fmt = args.join(' ');
    const re = /{([^}]+):([^}]+)}/g;
    const result = fmt.replace(re, (_match, color, msg) => {
        return rgb(color, msg);
    });
    console.log(result);
}
