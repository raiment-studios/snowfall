import { shell } from "../shell.ts";
import { rgb } from "../cprint.ts";
import { upgrade_tools } from "./upgrade_tools.ts";

export async function ensure_tools() {
  const root = Deno.env.get("MONOREPO_ROOT");
  if (!root) {
    console.error("MONOREPO_ROOT is not set.");
    Deno.exit(1);
  }
  Deno.chdir(root);

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
