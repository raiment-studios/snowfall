import React, { JSX } from 'react';
import { CSSString, useCSS } from './use_css.tsx';

export function Div({
    css,
    cl,
    className,
    style,
    children,

    onClick,
}: {
    css?: CSSString;
    cn?: string;
    cl?: string;
    className?: string;
    style?: React.CSSProperties;
    children?: React.ReactNode;

    onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
}): JSX.Element {
    const computedClassName =
        [cl, className, useCSS(css)]
            .filter((s) => !!s)
            .join(' ')
            .trim() || undefined;
    return (
        <div className={computedClassName} style={style} onClick={onClick}>
            {children}
        </div>
    );
}

export const D = Div;
