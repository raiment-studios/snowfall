// This is a very trivial, not-generalized server to provide the minimal
// endpoints for prototyping the applications.
//
// This server is safe by no means and should not be used outside development
// environments.

import { serve } from 'https://deno.land/std@0.203.0/http/server.ts';
import {
    parse as parseYAML,
    stringify as stringifyYAML,
} from 'https://deno.land/std@0.203.0/yaml/mod.ts';

const PORT = 3000;

async function handler(req: Request): Promise<Response> {
    const url = new URL(req.url);

    if (url.pathname !== '/build.timestamp') {
        console.log(`${req.method} ${url.pathname}`);
    }

    if (url.pathname === '/' && req.method === 'GET') {
        try {
            const indexHtml = await Deno.readTextFile('./dist/index.html');
            return new Response(indexHtml, {
                headers: { 'Content-Type': 'text/html' },
            });
        } catch (error) {
            console.error(error);
            return new Response('Failed to load index.html', { status: 500 });
        }
    } else if (
        url.pathname === '/main.bundle.js' ||
        url.pathname === '/main.bundle.js.map' ||
        url.pathname === '/build.timestamp'
    ) {
        const pathname = url.pathname.replace(/^\//, '');
        try {
            const content = await Deno.readTextFile(`./dist/${pathname}`);
            return new Response(content, {
                headers: { 'Content-Type': 'text/javascript' },
            });
        } catch (error) {
            console.error(error);
            return new Response('Failed to load file', { status: 500 });
        }
    } else if (url.pathname === '/api/read' && req.method === 'POST') {
        // Load YAML file
        try {
            const { path } = await req.json();
            const yamlContent = await Deno.readTextFile(`./data/${path}`);
            const parsedData = parseYAML(yamlContent);
            return new Response(JSON.stringify(parsedData), {
                headers: { 'Content-Type': 'application/json' },
            });
        } catch (error) {
            console.error(error);
            return new Response('Failed to load YAML file', { status: 500 });
        }
    } else if (url.pathname === '/api/write' && req.method === 'POST') {
        // Save YAML file
        try {
            const { path, content } = await req.json();
            const yamlContent = stringifyYAML(content);
            await Deno.writeTextFile(`./data/${path}`, yamlContent);
            return new Response('File saved successfully', { status: 200 });
        } catch (error) {
            console.error(error);
            return new Response('Failed to save YAML file', { status: 500 });
        }
    } else {
        return new Response('Not Found', { status: 404 });
    }
}

console.log(`HTTP server is running on http://localhost:${PORT}`);
await serve(handler, { port: PORT });
