import { command_validate_commit_msg } from "./commands/validate_commit_msg.ts";
import { change_directory } from "./commands/change_directory.ts";
import { command_cprintln } from "./commands/cprintln.ts";
import { command_system } from "./commands/command_system.ts";
import { upgrade_tools } from "./commands/upgrade_tools.ts";
import { ensure_tools } from "./commands/ensure_tools.ts";

async function main(args: string[]) {
  const table: Record<string, () => void> = {
    cprintln: () => command_cprintln(args.slice(1)),
    cd: () => change_directory(args.slice(1)),
    "validate-commit-msg": () => command_validate_commit_msg(args[1]),
    system: () => command_system(),
    "upgrade-tools": () => upgrade_tools(),
    versions: () => ensure_tools(),
  };

  const handler = table[args[0]];
  if (handler === undefined) {
    console.log("Unknown command:", args[0]);
    console.log("Known commands:");
    console.log("  " + Object.keys(table).sort().join("\n  "));
    Deno.exit(1);
  }
  handler();
}

await main(Deno.args);
