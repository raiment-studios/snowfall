import { shell } from "./shell.ts";
import { rgb } from "./cprint.ts";
import { command_validate_commit_msg } from "./commands/validate_commit_msg.ts";
import { change_directory } from "./commands/change_directory.ts";
import { command_cprintln } from "./commands/cprintln.ts";
import { command_system } from "./commands/command_system.ts";
import { upgrade_tools } from "./commands/upgrade_tools.ts";

async function main(args: string[]) {
  const table: Record<string, () => void> = {
    cprintln: () => command_cprintln(args.slice(1)),
    cd: () => change_directory(args.slice(1)),
    "validate-commit-msg": () => command_validate_commit_msg(args[1]),
    system: () => command_system(),
    "upgrade-tools": () => upgrade_tools(),
    versions: () => command_ensure_tools(),
  };

  const handler = table[args[0]];
  if (handler === undefined) {
    console.log("Unknown command:", args[0]);
    console.log("Known commands:");
    console.log("  " + Object.keys(table).join(" \n"));
    Deno.exit(1);
  }
  handler();
}

async function command_ensure_tools() {
  // Read .tool-versions file.  This is a file used by "asdf" to install local version
  // of the tools. This is the "source of truth" for the versions of the tools the
  // repository expects.
  const tool_versions: Record<string, string> = {};
  const s = (await Deno.readTextFile(".tool-versions")).trim();
  for (const line of s.split("\n").filter((s) => !!s.trim())) {
    const [tool, version] = line.split(" ");
    tool_versions[tool] = version;
  }

  const code = (await ensure_tool_versions(tool_versions)) ? 0 : 1;
  Deno.exit(code);
}

async function ensure_tool_versions(
  tool_versions: Record<string, string>
): Promise<boolean> {
  const unchecked_tools = ["bash", "asdf", "git", "git-lfs"];

  console.log(rgb("fff", "Binaries:"));
  for (const tool of unchecked_tools) {
    const version = await get_version(tool);
    console.log(
      " ",
      rgb("key", `${tool}`.padEnd(12, " ")), //
      rgb("#acaacc", `v${version}`.padEnd(12, " ")),
      rgb("#555", "-")
    );
  }

  let all_match = true;
  for (const [tool, version] of Object.entries(tool_versions)) {
    const actual_version = await get_version(tool);
    const match = actual_version === version;
    all_match &&= match;

    console.log(
      " ",
      rgb("key", `${tool}`.padEnd(12, " ")), //
      rgb("#acaacc", `v${actual_version}`.padEnd(12, " ")),
      !match //
        ? rgb("#f00", `${actual_version} != ${actual_version}`)
        : rgb("#0c0", "✓")
    );
  }

  if (!all_match) {
    console.log("Version mismatch, reinstalling all tools.");
    await upgrade_tools();
  }
  return all_match;
}

async function get_version(cmd: string) {
  try {
    switch (cmd) {
      case "golang": {
        const s = (await shell.spawn("go", ["version"])).stdout.split("\n")[0];
        const m = s.match(/(\d+\.\d+\.\d+)/);
        if (!m) {
          console.error("Could not read version for", cmd);
          console.error(s);
        }
        return m![1];
      }
      case "zig": {
        try {
          return (await shell.spawn(cmd, ["version"])).stdout.split("\n")[0];
        } catch (e) {
          return "";
        }
      }
      case "rust": {
        const s = (await shell.spawn("rustc", ["--version"])).stdout.split(
          "\n"
        )[0];
        const m = s.match(/(\d+\.\d+\.\d+)/);
        return m![1];
      }

      default: {
        const s = (await shell.spawn(cmd, ["--version"])).stdout.split("\n")[0];
        const m = s.match(/(\d+\.\d+\.\d+)/);
        return m![1];
      }
    }
  } catch (_) {
    return "";
  }
}

main(Deno.args);
