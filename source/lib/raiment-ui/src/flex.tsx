import React, { JSX } from 'react';
import { CSSString, useCSS } from './use_css.tsx';

export function Flex({
    row,
    col,
    flex,
    g,
    gap,
    align,
    w,
    h,
    minHeight,
    m,
    style,
    css,
    className,
    onClick,
    children,
}: {
    row?: boolean;
    col?: boolean;
    flex?: React.CSSProperties['flex'];
    g?: React.CSSProperties['flexGrow'];
    gap?: React.CSSProperties['gap'];
    align?: React.CSSProperties['alignItems'];
    w?: React.CSSProperties['width'];
    h?: React.CSSProperties['height'];
    minHeight?: React.CSSProperties['minHeight'];
    m?: React.CSSProperties['margin'];
    style?: React.CSSProperties;
    css?: CSSString;
    className?: string;

    onClick?: (evt: React.MouseEvent<HTMLDivElement>) => void;

    children?: React.ReactNode;
}): JSX.Element {
    const computed: React.CSSProperties = {
        display: 'flex',
        flexDirection: 'row',
        alignItems: 'center',
    };
    if (row) {
        computed.flexDirection = 'row';
        computed.alignItems = 'center';
    }
    if (col) {
        computed.flexDirection = 'column';
        computed.alignItems = 'stretch';
    }
    if (flex !== undefined) {
        computed.flex = flex;
    }
    if (g !== undefined) {
        computed.flexGrow = g;
    }
    if (align !== undefined) {
        computed.alignItems = align;
    }
    if (gap !== undefined) {
        computed.gap = gap;
    }
    if (w !== undefined) {
        computed.width = w;
    }
    if (h !== undefined) {
        computed.height = h;
    }
    if (minHeight !== undefined) {
        computed.minHeight = minHeight;
    }
    if (m !== undefined) {
        computed.margin = m;
    }
    if (style !== undefined) {
        Object.assign(computed, style);
    }

    const customClass = useCSS(css);
    return (
        <div className={`${className ?? ''} ${customClass}`} style={computed} onClick={onClick}>
            {children}
        </div>
    );
}
