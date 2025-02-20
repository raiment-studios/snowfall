import React, { JSX } from 'react';
import { CSSString, useCSS } from './use_css.tsx';

export function Div({
    css,
    cl,
    children,
}: {
    css?: CSSString;
    cl?: string;
    children?: React.ReactNode;
}): JSX.Element {
    const className = [cl, useCSS(css)].filter((s) => !!s).join(' ');
    return <div className={className}>{children}</div>;
}
