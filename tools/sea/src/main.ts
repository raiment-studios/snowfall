import { shell } from './shell.ts';
import { rgb } from './cprint.ts';
import { command_validate_commit_msg } from './commands/validate_commit_msg.ts';
import { change_directory } from './commands/change_directory.ts';
import { command_cprintln } from './commands/cprintln.ts';
import { command_system } from './commands/command_system.ts';

async function main(args: string[]) {
    switch (args[0]) {
        case 'cprintln':
            return command_cprintln(args.slice(1));
        case 'cd':
            return await change_directory(args.slice(1));
        case 'validate-commit-msg':
            return await command_validate_commit_msg(args[1]);
        case 'system':
            return await command_system();
        case 'upgrade-tools':
            return await upgrade_tools();
        case 'versions':
            return await command_ensure_tools();
    }
}

async function upgrade_tools() {
    console.error(
        `
#!/bin/env bash
echo "Refreshing asdf plugins..."
asdf plugin list all > /dev/null

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

asdf plugin-add bat
asdf install bat latest
asdf local bat latest

asdf plugin-add just
asdf install just latest
asdf local just latest

`.trim() + '\n'
    );
}

async function command_ensure_tools() {
    // Read .tool-versions file.  This is a file used by "asdf" to install local version
    // of the tools. This is the "source of truth" for the versions of the tools the
    // repository expects.
    const tool_versions: Record<string, string> = {};
    const s = (await Deno.readTextFile('.tool-versions')).trim();
    for (const line of s.split('\n').filter((s) => !!s.trim())) {
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
            rgb('key', `${tool}`.padEnd(8, ' ')), //
            rgb('#acaacc', `v${version}`.padEnd(12, ' ')),
            !match //
                ? rgb('#f00', `${actual_version} != ${actual_version}`)
                : rgb('#0c0', '✓')
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
                const s = (await shell.spawn('go', ['version'])).stdout.split('\n')[0];
                const m = s.match(/(\d+\.\d+\.\d+)/);
                if (!m) {
                    console.error('Could not read version for', cmd);
                    console.error(s);
                }
                return m![1];
            }
            case 'zig': {
                try {
                    return (await shell.spawn(cmd, ['version'])).stdout.split('\n')[0];
                } catch (e) {
                    return '';
                }
            }
            case 'rust': {
                const s = (await shell.spawn('rustc', ['--version'])).stdout.split('\n')[0];
                const m = s.match(/(\d+\.\d+\.\d+)/);
                return m![1];
            }

            default: {
                const s = (await shell.spawn(cmd, ['--version'])).stdout.split('\n')[0];
                const m = s.match(/(\d+\.\d+\.\d+)/);
                return m![1];
            }
        }
    } catch (_) {
        return '';
    }
}

main(Deno.args);
