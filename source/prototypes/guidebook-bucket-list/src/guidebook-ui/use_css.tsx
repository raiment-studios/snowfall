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
        if (!className) {
            return;
        }
        const id = `id-${className}`;
        let el = document.getElementById(id);
        if (el) {
            el.dataset.count = `${parseInt(el.dataset.count ?? '0') + 1}`;
            return;
        }
        el = document.createElement('style');
        el.innerHTML = `.${className} { ${s} }`;
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
