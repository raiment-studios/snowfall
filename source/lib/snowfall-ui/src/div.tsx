import React, { JSX } from 'react';
import { CSSString, useCSS } from './use_css.tsx';

export function Div({
    css,
    cn,
    children,
}: {
    css?: CSSString;
    cn?: string;
    children?: React.ReactNode;
}): JSX.Element {
    const className = [cn, useCSS(css)].filter((s) => !!s).join(' ');
    return <div className={className}>{children}</div>;
}
