# ❄️ snowtale-tables

A client-side random generator for the Galthea universe.

This may or may not be a starting point for a Snowtale journaling RPG.

## FAQ

#### Why does this use Node.js? I thought the tech stack was Deno-based.

Ideally the tech stack would be limited to `Deno` rather than `Node`. However, we're not aware of a "better" (for some definition of that word) web bundler than esbuild -- and that does not seem to be fully Deno compatible (requires plug-ins for JSR imports, etc). Until there's a cleaner solution (that someone on the team is aware of!), using Node and NPM with esbuild seems to be the path of least resistance to spent more time on the code than the tooling.
