import { cprintln } from '../cprint.ts';

export async function command_system() {
    cprintln();
    cprintln('key', `❄️ snowfall development environment`);
    cprintln('#555', '~'.repeat(80));
    cprintln();

    const os = Deno.build.os;
    const arch = Deno.build.arch;
    const mem = Deno.systemMemoryInfo();

    const os_name =
        {
            darwin: 'Mac',
            linux: 'Linux',
            windows: 'Windows',
        }[os as string] ?? 'unknown';
    cprintln('key', `System: ${os_name} ${arch} ${mem.total / Math.pow(1024, 3)} GiB`);
}