# snowfall

[![license: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/) [![status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-purple.svg)](http://unlicense.org/) ![Tests](https://github.com/raiment-studios/snowfall/actions/workflows/ci.yml/badge.svg) [![Bluesky](https://img.shields.io/badge/Bluesky-0285FF?logo=bluesky&logoColor=fff&style=flat)](https://bsky.app/profile/ridleywinters.bsky.social)

Snowfall is a broad project:

-   It is a open source voxel engine
-   It is a open source content repository
-   It is a set of open source games
-   It is a standardized set of tools to make contribution & support easier

For news and more info, see the following social accounts:

## Goal of Snowfall

Publish an **open-source**, **voxel-based**, **infinite world** **exploration and adventure** game that is **easy and enjoyable to contribute to** for all levels of expertise. Provide a **standard distribution** of content and mods while also encouraging **forked distributions** tailored to different goals.

Ensuring participating in _development and evolution_ of Snowfall is enjoyable is as important a goal as is making the game itself fun.

## Current status

In very, very early development!

If you're looking to contribute see [TODO.md](TODO.md).

## Development

Clone the repository and **source `setup.sh`** to set up the environment:

```bash
git clone git@github.com:raiment-studios/snowfall.git
cd snowfall
source setup.sh
```

Note `setup.sh` does a non-trivial amount of setup to attempt to ensure a common development environment for all users.

## Components

### snowglobe

Snowglobe is the primary game within the snowfall project. It is the voxel-based incarnation of the game.

### snowtale

A text-based variation of the snowfall project that due to its text-based interface is used to define and improve the story-building aspects of the snowfall project as a whole.

### galthea

Galthea is the name of the fictional universe in which the snowfall games take place. The content created for these projects aims to be internally consistent with the style and lore of Galthea.

## Development

The development of Snowfall is utilized primarily **Rust** and, secondarily, **Deno** for cases where a scripting language is a more apt fit.

The Snowfall project "enforces" quite a few standards in the development to normalize what tools, versions, and development methods are used; this intention of these restrictions is to make participating in development and getting support as simple as possible.
