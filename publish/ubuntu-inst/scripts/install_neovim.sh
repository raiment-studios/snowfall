#!/usr/bin/env bash
pushd ~
mkdir -p sources
cd sources
git clone https://github.com/neovim/neovim
cd neovim
make CMAKE_BUILD_TYPE=RelWithDebInfo
sudo make install
popd