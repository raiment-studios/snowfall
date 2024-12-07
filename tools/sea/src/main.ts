import { rgb24 } from 'jsr:@gnome/ansi@0.2';

import { Shell } from './shell.ts';
const sh = new Shell();

function cprintln(color?: string, msg?: string) {
    if (!msg) {
        console.log();
        return;
    }

    function parseColor(color: string): { r: number; g: number; b: number } {
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

    let s = rgb24(msg, parseColor(color));
    console.log(s);
}

async function main(args: string[]) {
    switch (args[0]) {
        case 'system':
            return await command_system();
        case 'upgrade-tools':
            return await upgrade_tools();
        case 'versions':
            return await command_ensure_tools();
    }
}

async function command_system() {
    cprintln();
    cprintln('#3b23d7', `❄️ snowfall development environment`);
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
    cprintln('#3b23d7', `System: ${os_name} ${arch} ${mem.total / Math.pow(1024, 3)} GiB`);
}

async function upgrade_tools() {
    console.error(
        `
#!/bin/env bash
asdf plugin-add deno https://github.com/asdf-community/asdf-deno.git
asdf install deno latest
asdf local deno latest

asdf plugin-add rust https://github.com/asdf-community/asdf-rust.git
asdf install rust latest
asdf local rust latest


asdf plugin-add zig https://github.com/asdf-community/asdf-zig.git
asdf install zig latest
asdf local zig latest

asdf plugin add golang https://github.com/asdf-community/asdf-golang.git
asdf install golang latest
asdf local golang latest

asdf plugin add starship
asdf install starship latest
asdf local starship latest

asdf plugin-add zellij
asdf install zellij latest
asdf local zellij latest
`.trim() + '\n'
    );
}

async function command_ensure_tools() {
    // Read .tool-versions file.  This is a file used by "asdf" to install local version
    // of the tools. This is the "source of truth" for the versions of the tools the
    // repository expects.
    const tool_versions: Record<string, string> = {};
    const s = (await Deno.readTextFile('.tool-versions')).trim();
    for (const line of s.split('\n')) {
        const [tool, version] = line.split(' ');
        tool_versions[tool] = version;
    }

    const code = (await ensure_tool_versions(tool_versions)) ? 0 : 1;
    Deno.exit(code);
}

async function ensure_tool_versions(tool_versions: Record<string, string>): Promise<boolean> {
    let all_match = true;
    for (const [tool, version] of Object.entries(tool_versions)) {
        const actual_version = await get_version(tool);
        const match = actual_version === version;
        all_match &&= match;

        console.log(
            '  ',
            rgb24(`${tool}`.padEnd(8, ' '), parseColor('#3b23d7')), //
            rgb24(`v${version}`.padEnd(12, ' '), parseColor('#acaacc')),
            !match
                ? rgb24(`${actual_version} != ${actual_version}`, parseColor('#f00'))
                : rgb24('✓', parseColor('#0c0'))
        );
    }

    if (!all_match) {
        console.log('Version mismatch, reinstalling all tools.');
        await upgrade_tools();
    }
    return all_match;
}

async function get_version(cmd: string) {
    try {
        switch (cmd) {
            case 'golang': {
                const s = (await sh.spawn('go', ['version'])).stdout.split('\n')[0];
                const m = s.match(/(\d+\.\d+\.\d+)/);
                if (!m) {
                    console.error('Could not read version for', cmd);
                    console.error(s);
                }
                return m![1];
            }
            case 'zig': {
                try {
                    return (await sh.spawn(cmd, ['version'])).stdout.split('\n')[0];
                } catch (e) {
                    return '';
                }
            }
            case 'rust': {
                const s = (await sh.spawn('rustc', ['--version'])).stdout.split('\n')[0];
                const m = s.match(/(\d+\.\d+\.\d+)/);
                return m![1];
            }

            default: {
                const s = (await sh.spawn(cmd, ['--version'])).stdout.split('\n')[0];
                const m = s.match(/(\d+\.\d+\.\d+)/);
                return m![1];
            }
        }
    } catch (_) {
        return '';
    }
}

main(Deno.args);

function parseColor(color: string): { r: number; g: number; b: number } {
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
