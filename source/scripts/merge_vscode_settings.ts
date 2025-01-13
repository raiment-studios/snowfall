import { parse } from 'jsr:@std/jsonc';
import { deepMerge } from 'jsr:@std/collections';

async function main() {
    const root = Deno.env.get('MONOREPO_ROOT');
    if (!root) {
        console.error('MONOREPO_ROOT is not set');
        Deno.exit(1);
    }

    // Get the command value of "git config user.email"
    const email = (await spawn('git', ['config', 'user.email'])).stdout.trim();
    const default_config = parse(
        await Deno.readTextFile(`${root}/config/profiles/_default/settings.json`)
    );
    const user_config = parse(
        await Deno.readTextFile(`${root}/config/profiles/${email}/settings.json`)
    );

    const config = deepMerge(default_config, user_config);

    await Deno.writeTextFile(`${root}/.vscode/settings.json`, JSON.stringify(config, null, 2));
}
await main();

// Runs the given command and captures the output.
async function spawn(command: string, args: string[]) {
    const opts: Deno.CommandOptions = {
        args,
    };
    opts.stdout = 'piped';
    opts.stderr = 'piped';

    const cmd = new Deno.Command(command, opts);
    const proc = cmd.spawn();
    const output = await proc.output();
    const stdout = new TextDecoder().decode(output.stdout);
    const stderr = new TextDecoder().decode(output.stderr);
    return {
        success: output.success,
        stdout,
        stderr,
    };
}
