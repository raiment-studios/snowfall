import { shell } from '../shell.ts';
import { cprintln, rgb } from '../cprint.ts';

export async function command_validate_commit_msg(filename: string) {
    const message = Deno.readTextFileSync(filename);

    const validTypeDescs = {
        feat: 'a new feature or improvement in user experience',
        fix: 'a bug fix',
        docs: 'documentation changes',
        arch: 'refactor or internal architectural improvements',
        perf: 'a code change that improves performance',
        test: 'add or improve tests',
        build: 'changes to the build or deployment system',
        misc: 'escape hatch for anything that does not fit into the above',
    };
    const printValidTypes = () => {
        cprintln('999', 'Valid types:');
        for (const [type, desc] of Object.entries(validTypeDescs)) {
            console.error(`  ${rgb('key', type.padStart(5, ' '))}: ${rgb('999', desc)}`);
        }
    };

    if (typeof message !== 'string') {
        console.error('Commit message must be a string');
        Deno.exit(1);
    }

    const last = (await shell.spawn('git', ['log', '-n', '1', '--pretty=%B'])).stdout;
    const scrub = (s: string) => s.toLocaleLowerCase().replace(/\s/g, '');
    if (scrub(last) === scrub(message)) {
        cprintln('error', 'Commit message is the same as the last commit message');
        Deno.exit(1);
    }

    const validTypes = Object.keys(validTypeDescs);

    // Split the message into the part before ":" and everything after
    const parts = message.split(':');
    if (parts.length === 1) {
        cprintln('F00', 'Commit message must contain a <type>: <message>');
        cprintln();
        printValidTypes();
        Deno.exit(1);
    }
    const [type, _description] = [parts[0], parts.slice(1).join(':')];
    if (!validTypes.includes(type)) {
        console.error(`Invalid commit type: ${type}`);
        printValidTypes();
        Deno.exit(1);
    }

    // All good!
    Deno.exit(0);
}
