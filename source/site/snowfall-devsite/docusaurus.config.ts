//
// This all runs in Node.js - Don't use client-side code here (browser APIs, JSX...)
//
import { themes as prismThemes } from 'prism-react-renderer';
import type { Config, MarkdownConfig } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import type { ElementContent } from 'hast';

// NOTE: not sure if this going to work as it scales -- docusaurus might process multiple
// pages in parallel so it won't necessarily be fully built by the time it is first used.
const globalAliasMap: Record<string, string> = {};

class ElementBuilder {
    _elements: ElementContent[] = [];

    get elements() {
        return this._elements;
    }

    text(value: string) {
        this._elements.push({
            type: 'text',
            value,
        });
    }

    a(href: string, cb: (builder: ElementBuilder) => void) {
        const builder = new ElementBuilder();
        cb(builder);

        this._elements.push({
            type: 'element',
            tagName: 'a',
            properties: {
                href,
            },
            children: builder.elements,
        });
    }
    span(value: string, properties: Record<string, any> = {}) {
        this._elements.push({
            type: 'element',
            tagName: 'span',
            properties,
            children: [{ type: 'text', value }],
        });
    }
}

// Note: this is a bit hacky as I'm not a novice regarding Docusaurus and this has been
// written for expediency, not elegance.
const markdownCustomization: MarkdownConfig = {
    format: 'mdx',
    mermaid: true,
    preprocessor: ({ filePath, fileContent }) => {
        const _filePath = filePath;
        return fileContent;
    },
    parseFrontMatter: async (params) => {
        // Reuse the default parser
        const { filePath } = params;
        const result = await params.defaultParseFrontMatter(params);
        const frontMatter: { [key: string]: any } = result.frontMatter ?? {};

        frontMatter.filePath = filePath;
        frontMatter.alias = (filePath.split('/').pop() ?? '').toLowerCase();
        frontMatter.alias = frontMatter.alias.replace(/\.[^.]+$/, '');
        globalAliasMap[frontMatter.alias] = frontMatter.filePath;

        result.frontMatter = frontMatter;
        return result;
    },
    mdx1Compat: {
        comments: true,
        admonitions: true,
        headingIds: true,
    },
    remarkRehypeOptions: {
        handlers: {
            text: (state, text) => {
                //
                // Check for "[[wiki-words]]" content within text
                //
                const builder = new ElementBuilder();
                let buffer = text.value;
                while (buffer.length > 0) {
                    const m = buffer.match(/\[\[([a-zA-Z0-9_-]+?)\]\]/);
                    if (m) {
                        builder.text(buffer.slice(0, m.index));

                        const value = m[1];
                        const filePath = globalAliasMap[value] ?? '';
                        if (filePath) {
                            let href = filePath.slice(state.options.file?.cwd.length);
                            href = href.replace(/\.[^.]+$/, '');
                            builder.a(href, (children) => children.text(value));
                        } else {
                            builder.span(value, { style: 'color: red' });
                        }

                        buffer = buffer.slice(m.index + m[0].length);
                    } else {
                        builder.text(buffer);
                        buffer = '';
                    }
                }
                return builder.elements;
            },
        },
    },
    anchors: {
        maintainCase: true,
    },
};

const config: Config = {
    title: 'snowfall-devsite',
    tagline: 'Snowfall Internal Development Site',
    favicon: 'img/favicon.ico',
    url: 'https://raiment-studios.com',
    // Set the /<baseUrl>/ pathname under which your site is served
    // For GitHub pages deployment, it is often '/<projectName>/'
    baseUrl: '/',

    // GitHub pages deployment config.
    // If you aren't using GitHub pages, you don't need these.
    organizationName: 'raiment-studios', // Usually your GitHub org/user name.
    projectName: 'snowfall', // Usually your repo name.
    onBrokenLinks: 'ignore',
    onBrokenMarkdownLinks: 'warn',

    // Even if you don't use internationalization, you can use this field to set
    // useful metadata like html lang. For example, if your site is Chinese, you
    // may want to replace "en" with "zh-Hans".
    i18n: {
        defaultLocale: 'en',
        locales: ['en'],
    },

    presets: [
        [
            'classic',
            {
                docs: {
                    sidebarPath: './sidebars.ts',
                    // Please change this to your repo.
                    // Remove this to remove the "edit this page" links.
                    editUrl:
                        'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
                },
                blog: {
                    showReadingTime: true,
                    feedOptions: {
                        type: ['rss', 'atom'],
                        xslt: true,
                    },
                    // Please change this to your repo.
                    // Remove this to remove the "edit this page" links.
                    editUrl:
                        'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
                    // Useful options to enforce blogging best practices
                    onInlineTags: 'warn',
                    onInlineAuthors: 'warn',
                    onUntruncatedBlogPosts: 'warn',
                },
                theme: {
                    customCss: './src/css/custom.css',
                },
            } satisfies Preset.Options,
        ],
    ],

    themeConfig: {
        // Replace with your project's social card
        image: 'img/docusaurus-social-card.jpg',
        navbar: {
            title: 'My Site',
            logo: {
                alt: 'My Site Logo',
                src: 'img/logo.svg',
            },
            items: [
                {
                    type: 'docSidebar',
                    sidebarId: 'tutorialSidebar',
                    position: 'left',
                    label: 'Tutorial',
                },
                { to: '/blog', label: 'Blog', position: 'left' },
                {
                    href: 'https://github.com/facebook/docusaurus',
                    label: 'GitHub',
                    position: 'right',
                },
            ],
        },
        footer: {
            style: 'dark',
            links: [
                {
                    title: 'Docs',
                    items: [
                        {
                            label: 'Tutorial',
                            to: '/docs/intro',
                        },
                    ],
                },
                {
                    title: 'Community',
                    items: [
                        {
                            label: 'Stack Overflow',
                            href: 'https://stackoverflow.com/questions/tagged/docusaurus',
                        },
                        {
                            label: 'Discord',
                            href: 'https://discordapp.com/invite/docusaurus',
                        },
                        {
                            label: 'X',
                            href: 'https://x.com/docusaurus',
                        },
                    ],
                },
                {
                    title: 'More',
                    items: [
                        {
                            label: 'Blog',
                            to: '/blog',
                        },
                        {
                            label: 'GitHub',
                            href: 'https://github.com/facebook/docusaurus',
                        },
                    ],
                },
            ],
            copyright: `Copyright © ${new Date().getFullYear()} My Project, Inc. Built with Docusaurus.`,
        },
        prism: {
            theme: prismThemes.github,
            darkTheme: prismThemes.dracula,
        },
    } satisfies Preset.ThemeConfig,

    markdown: markdownCustomization,
};

export default config;
