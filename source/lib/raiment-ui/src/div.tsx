import React, { JSX } from 'react';
import { CSSString, useCSS } from './use_css.tsx';

export function Div({
    css,
    cl,
    className,
    children,
}: {
    css?: CSSString;
    cn?: string;
    cl?: string;
    className?: string;
    children?: React.ReactNode;
}): JSX.Element {
    const computedClassName = [cl, className, useCSS(css)].filter((s) => !!s).join(' ');
    return <div className={computedClassName}>{children}</div>;
}

export const D = Div;
