import React from 'react';

// Returns the access token
//
// TODO:
// - Token expiration is not considered
export function useGitHubAuthToken(): string | null {
    // Get the existing value if there is one
    const accessToken = localStorage.getItem('github_auth/access_token');

    React.useLayoutEffect(() => {
        const url = new URL(window.location.href);

        // --- If there's already an access token... ---
        //
        // We're already authenticated, so update the state and return
        // it to the caller. If we just got the access token, clean up
        // the URL to remove our handshake parameters.
        //
        if (accessToken) {
            // Clear any auth callback parameters from the URL if necessary
            // and reload.
            const scrubbed = new URL(window.location.href);
            scrubbed.searchParams.delete('auth');
            scrubbed.searchParams.delete('code');
            scrubbed.searchParams.delete('state');
            if (scrubbed.toString() !== url.toString()) {
                window.location.href = scrubbed.toString();
            }
            return;
        }

        const auth = url.searchParams.get('auth');
        const code = url.searchParams.get('code');

        // --- Callback from GitHub after sign-in? ---
        //
        // Redirect to the guidebook auth server to get an access token from
        // the "code" provided by GitHub.  We need the indirect auth server to
        // avoid CORS issues (GitHub won't accept the callback from the browser).
        //
        if (auth === 'github' && code?.length) {
            const isLocalAuth = window.location.hostname === 'localhost';
            const clientID = isLocalAuth ? 'Ov23lilAyyeHVnqZ1pGc' : 'Ov23li89ZvKkoY3YqFDj';

            console.log('Resolving GitHub auth callback', auth, code, clientID);
            const go = async () => {
                const url = new URL('https://guidebook-auth-server.deno.dev/');
                const body = {
                    url: `https://github.com/login/oauth/access_token`,
                    method: 'POST',
                    headers: {
                        Accept: 'application/json',
                        'Content-Type': 'application/json',
                    },
                    body: {
                        client_id: clientID,
                        code,
                    },
                };

                const resp = await fetch(url.toString(), {
                    method: 'POST',
                    headers: {
                        Accept: 'application/json',
                        'Content-Type': 'application/json',
                        'cache-control': 'no-cache',
                    },
                    body: JSON.stringify(body),
                });

                const json = await resp.json();
                if (json.access_token) {
                    console.log('Got access token', json);
                    localStorage.setItem('github_auth/access_token', json.access_token);
                    window.location.reload();
                    return;
                }
            };

            go();
        }

        // Otherwise, we don't have an access token or a code, so there's nothing
        // for this hook to do!
    }, []);

    return accessToken ?? '';
}

export class GitHubAPI {
    _token: string;

    _cache: { [key: string]: any } = {};

    constructor(token: string) {
        this._token = token;
    }

    get token(): string {
        return this._token;
    }

    async user() {
        return this.cachedFetch(`https://api.github.com/user`);
    }

    async fetchRaw(method: string, url: string, body: any = null): Promise<Response> {
        return await fetch(url, {
            method,
            headers: {
                Authorization: `Bearer ${this._token}`,
                Accept: 'application/vnd.github+json',
                'X-GitHub-Api-Version': '2022-11-28',
                'User-Agent': 'raiment-studios-guidebook',
            },
            body: body ? JSON.stringify(body) : undefined,
        });
    }

    async fetch(method: string, url: string, body: any = null): Promise<any> {
        const resp = await this.fetchRaw(method, url, body);
        if (!resp.ok) {
            return null;
        }
        const json = await resp.json();
        return json;
    }

    async cachedFetch(url: string): Promise<any> {
        const cached = this._cache[url];
        if (cached) {
            return cached;
        }
        const json = await this.fetch('GET', url);
        this._cache[url] = json;
        return json;
    }

    async repositoryExists(repositoryName: string): Promise<boolean> {
        const user = await this.user();
        const username = user.login;
        const url = `https://api.github.com/repos/${username}/${repositoryName}`;
        try {
            const resp = await this.fetchRaw('GET', url);
            return resp.status === 200 ? true : false;
        } catch (_err) {
            return false;
        }
    }

    async createRepository(repositoryName: string): Promise<void> {
        const url = `https://api.github.com/user/repos`;
        const params = {
            name: repositoryName,
            description: 'Guidebook data repository',
            private: false,
            has_issues: false,
            has_projects: false,
            has_wiki: false,
            has_downloads: false,
            auto_init: true,
        };
        await this.fetch('POST', url, params);
    }

    async readFileContents(filename: string): Promise<string | null> {
        const user = await this.user();
        const username = user.login;
        const repo = 'guidebook-data';
        const url = `https://api.github.com/repos/${username}/${repo}/contents/${filename}`;

        const existing = await this.fetch('GET', url);
        if (!existing) {
            return null;
        }
        const encoded = existing.content;
        const content = atob(encoded);
        return content;
    }

    _updateTimers: Record<string, number | undefined> = {};

    async updateFileContents(filename: string, content: string, delay = 1500): Promise<void> {
        window.clearTimeout(this._updateTimers[filename]);

        this._updateTimers[filename] = window.setTimeout(async () => {
            const user = await this.user();
            const username = user.login;
            const repo = 'guidebook-data';
            const url = `https://api.github.com/repos/${username}/${repo}/contents/${filename}`;

            let sha;
            {
                const existing = await this.fetch('GET', url);
                if (existing) {
                    sha = existing.sha;
                }
            }

            // Base64 encode the content
            const encoded = btoa(content);
            this.fetch('PUT', url, {
                message: 'Update from guidebook-bucket-list app',
                committer: {
                    name: 'guidebook-bucket-list',
                    email: 'support@raiment-studios.com',
                },
                content: encoded,
                sha: sha,
            });

            this._updateTimers[filename] = undefined;
        }, delay);
    }
}

let _createOnce = false;
export function useGitHubAPI(): GitHubAPI | null {
    const accessToken = useGitHubAuthToken();
    const api = React.useMemo(() => {
        if (!accessToken) {
            return null;
        }
        const api = new GitHubAPI(accessToken);
        if (!_createOnce) {
            _createOnce = true;
            const name = 'guidebook-data';
            api.repositoryExists(name).then((exists) => {
                if (exists) {
                    return;
                }
                api.createRepository(name);
            });
        }
        return api;
    }, [accessToken]);

    return api;
}
