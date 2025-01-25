import React, { JSX } from 'react';
import { CSSString, useCSS } from './use_css.tsx';

export function Div({
    css,
    cn,
    cl,
    children,
}: {
    css?: CSSString;
    cn?: string;
    cl?: string;
    children?: React.ReactNode;
}): JSX.Element {
    const className = [cl, cn, useCSS(css)].filter((s) => !!s).join(' ');
    return <div className={className}>{children}</div>;
}
