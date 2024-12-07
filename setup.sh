#!/usr/bin/env bash

# =============================================================================
# Keep this file minimal as the goal to keep as much logic in Deno/Rust
# as possible.
# =============================================================================

# -----------------------------------------------------------------------------
# Environment variables
# -----------------------------------------------------------------------------

# Store the root of the repository as a well-known, "stable" reference
# environment variables which scripts can access other assets in the repo.
# https://stackoverflow.com/questions/59895/how-can-i-get-the-source-directory-of-a-bash-script-from-within-the-script-itsel
export MONOREPO_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# -----------------------------------------------------------------------------
# Install asdf to bootstrap things
# -----------------------------------------------------------------------------

which asdf > /dev/null
if [ $? -ne 0 ]; then
    git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.14.0    
fi
source "$HOME/.asdf/asdf.sh"
source "$HOME/.asdf/completions/asdf.bash"
source /opt/homebrew/opt/asdf/libexec/asdf.sh

# -----------------------------------------------------------------------------
# Aliases
# -----------------------------------------------------------------------------

eval "$(starship init bash)"

function gs() {
    git status
}

# Alias to give "sea" access to the source environment.  All stderr output from
# sea is treated as a script that is executed at the end of the command.
function sea() {
    eval "$MONOREPO_ROOT/tools/sea/sea $*" 2> $MONOREPO_ROOT/temp/__output.sh
    cat $MONOREPO_ROOT/temp/__output.sh | bash
    rm $MONOREPO_ROOT/temp/__output.sh
}

# For each file in $MONOREPO_ROOT/config/git-hooks, link to local hooks
for source in $MONOREPO_ROOT/config/git-hooks/*; do
    hook=$(basename $file)
    target=$MONOREPO_ROOT/.git/hooks/$hook    
    rm -f $target
    ln -s $source $target
    chmod +x $target
done



sea system
sea versions



