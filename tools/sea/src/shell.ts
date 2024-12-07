export class Shell {
    async command_exists(cmd: string): Promise<boolean> {
        const { success } = await this.run('which', [cmd]);
        return success;
    }

    /// Runs the given command streams any output to stdout.
    async run(command: string, args: string[]) {
        const opts: Deno.CommandOptions = {
            args,
        };
        const cmd = new Deno.Command(command, opts);
        const proc = cmd.spawn();
        const output = await proc.output();
        return {
            success: output.success,
        };
    }

    // Runs the given command and captures the output.
    async spawn(command: string, args: string[]) {
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
}

export const shell = new Shell();
