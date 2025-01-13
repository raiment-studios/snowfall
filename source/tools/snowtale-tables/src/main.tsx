import React, { JSX } from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './app.tsx';

function MainBootstrap(): JSX.Element {
    useLongPollRefresh('./main.bundle.js');
    return <App />;
}

async function main(): Promise<void> {
    const element = document.getElementById('root')!;
    const root = ReactDOM.createRoot(element);
    root.render(<MainBootstrap />);
}
main();

/**
 * Continuously polls the server at a given URL and reloads the page if the
 * contents change. A simple way to implement reloading on development changes.
 *
 * Note: this has a downside of spamming the web console with messages. A
 * server-side messaging approach does not have this downside, but requires a
 * server that is aware of the hot-reload strategy.
 */
function useLongPollRefresh(url: string) {
    // Poll the server for index.html continuously and check if the
    // contents changes. If it changes, reload the window.
    React.useEffect(() => {
        let cache: string | null = null;
        let timer: number | undefined;
        const delay = 250 + Math.floor(750 * Math.random());
        const check = async () => {
            const response = await fetch(url);
            if (response.ok) {
                const text = await response.text();
                if (cache !== null && text !== cache) {
                    location.reload();
                    return;
                }
                cache = text;
            }
            timer = setTimeout(check, delay);
        };
        timer = setTimeout(check, delay);
        return () => clearTimeout(timer);
    });
}
