#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

pushd ~

sudo apt update
sudo apt upgrade -y
sudo apt install -y \
    vim \
    curl wget \
    git git-lfs \
    coreutils unzip gzip bzip2 xz-utils tar \
    bash gcc make \
    ninja-build gettext cmake build-essential

ensure_line() {
    local line="$1"
    local file="$2"
    if ! grep -Fxq "$line" "$file"; then
        echo "$line" >> "$file"
    fi
}

git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.15.0

ensure_line ". \"$HOME/.asdf/asdf.sh\"" ".bashrc"
ensure_line ". \"$HOME/.asdf/completions/asdf.bash\"" ".bashrc"
source ~/.bashrc


asdf plugin-add rust https://github.com/asdf-community/asdf-rust.git
asdf plugin-add starship https://github.com/gr1m0h/asdf-starship

function _adsf_latest {
    local plugin=$1
    asdf plugin add $plugin
    asdf install $plugin latest
    asdf global $plugin latest
}


_adsf_latest starship
ensure_line "eval \"\$(starship init bash)\"" ".bashrc"
source ~/.bashrc

_adsf_latest rust
_adsf_latest deno
_adsf_latest golang
_adsf_latest zig

# The neovim installation by asdf doesn't seem to work within a Ubuntu
# docker container running on an M2 chip, so bulding from source.
bash $SCRIPT_DIR/install_neovim.sh

echo
echo
echo "---------------------------------------------------------------"
echo "Done!"
echo "---------------------------------------------------------------"
echo 

set -u _adsf_latest

popd