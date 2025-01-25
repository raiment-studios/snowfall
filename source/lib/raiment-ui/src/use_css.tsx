import React from 'react';

export class CSSString {
    value: string = '';
    constructor(value: string) {
        this.value = value;
    }
    toString(): string {
        return this.value;
    }
}

export function css(arr: TemplateStringsArray, ...values: string[]): CSSString {
    const results = [];
    results.push(arr[0]);
    for (let i = 0; i < values.length; i++) {
        results.push(values[i]);
        results.push(arr[i + 1]);
    }
    return new CSSString(results.join(''));
}

function hashString(input: string): number {
    let hash = 0;
    for (let i = 0; i < input.length; i++) {
        const charCode = input.charCodeAt(i);
        hash = (hash << 5) - hash + charCode;
        hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash);
}

export function useCSS(s?: CSSString): string {
    const className = React.useMemo(() => {
        if (!s) {
            return '';
        }
        return `_class_${hashString(s.toString()).toString(16)}`;
    }, [s]);

    React.useEffect(() => {
        if (!className || !s) {
            return;
        }
        const id = `id-${className}`;
        let el = document.getElementById(id);
        if (el) {
            el.dataset.count = `${parseInt(el.dataset.count ?? '0') + 1}`;
            return;
        }

        let text = s.toString();
        let isGlobal = false;
        {
            const re1 = /^\s*\.[a-zA-Z0-9_]+\s*\{/;
            const re2 = /\s*\}\s*$/;
            const m1 = re1.exec(text);
            const m2 = re2.exec(text);
            if (m1 && m2) {
                isGlobal = m1[0].includes('.global');
                text = text.substring(m1.index + m1[0].length);
                text = text.substring(0, text.length - m2[0].length);
            }
        }
        const lines = text.split('\n');
        while (lines.length > 0 && lines[0].trim().length === 0) {
            lines.shift();
        }
        while (lines.length > 0 && lines[lines.length - 1].trim().length === 0) {
            lines.pop();
        }

        let minIndent = Number.MAX_SAFE_INTEGER;
        for (const line of lines) {
            const m = /^\s*/.exec(line);
            if (line.trim().length > 0 && m) {
                minIndent = Math.min(minIndent, m[0].length);
            }
        }
        if (minIndent === Number.MAX_SAFE_INTEGER) {
            minIndent = 0;
        }
        for (let i = 0; i < lines.length; i++) {
            lines[i] = lines[i].substring(minIndent);
        }

        const generatedCSS = ['', ...lines].join('\n    ');
        const generated = isGlobal ? generatedCSS : `.${className} {${generatedCSS}\n}`;

        el = document.createElement('style');
        el.innerHTML = generated;
        el.dataset.count = '1';
        document.head.appendChild(el);

        return () => {
            const el = document.getElementById(id);
            if (!el) {
                return;
            }
            const count = parseInt(el.dataset.count ?? '0');
            el.dataset.count = `${count - 1}`;
            if (count <= 1) {
                el.remove();
            }
        };
    }, [className]);

    return className;
}
