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
# git setup
# -----------------------------------------------------------------------------

git lfs install > /dev/null

# For each file in $MONOREPO_ROOT/config/git-hooks, link to local hooks
for source in $MONOREPO_ROOT/config/git-hooks/*; do
    hook=$(basename $source)
    target=$MONOREPO_ROOT/.git/hooks/$hook    
    rm -f $target
    ln -s $source $target
    chmod +x $target
done

# -----------------------------------------------------------------------------
# Shell customizations
# -----------------------------------------------------------------------------

function add_to_path() {
    if [ -d "$1" ] && [[ ":$PATH:" != *":$1:"* ]]; then
        PATH="${PATH:+"$PATH:"}$1"
    fi
}

eval "$(starship init bash)"

add_to_path "$MONOREPO_ROOT/bin"
rm -f "$MONOREPO_ROOT/bin/sea"

# Alias to give "sea" access to the source environment.  All stderr output from
# sea is treated as a script that is executed at the end of the command.
unalias sea 2> /dev/null
unset -f sea
function sea() {
    eval $MONOREPO_ROOT/tools/sea/sea $@ 2> $MONOREPO_ROOT/temp/__output.sh
    cat $MONOREPO_ROOT/temp/__output.sh
    source $MONOREPO_ROOT/temp/__output.sh
    rm $MONOREPO_ROOT/temp/__output.sh
}

function cprint() {
    $MONOREPO_ROOT/tools/sea/sea cprintln "$@"
}

# Git Log (gslog)
unalias gslog 2> /dev/null
unset -f gslog
alias gslog='git log -n 12 --pretty="%C(auto)%h %C(#3fc9b4)%ad %as %C(green)%s%C(reset)" --color --date=format:"%a"'

# Git Status (gs)
unalias gs 2> /dev/null
unset -f gs
function gs() {
    cprint "{999:● Recent commits ●}"
    gslog
    echo
    cprint "{999:● Current git status ●}"
    git status
}

# Git Commit And Push (gcap)
unalias gcap 2> /dev/null
unset -f gcap
function gcap() {
    pushd $MONOREPO_ROOT
    git add .
    git commit -m "$*"
    git push
    popd
}

unalias scd 2> /dev/null
unset -f scd
function scd() {
    sea cd $*
}

if [ -f "$HOME/.config/snowfall-dev/profile.sh" ]; then
    source "$HOME/.config/snowfall-dev/profile.sh"
fi



sea system
sea versions

cprint 
cprint "{557:Writing merged .vscode/settings.json}"
deno --allow-all $MONOREPO_ROOT/source/scripts/merge_vscode_settings.ts 

cprint
cprint "Environment:"
cprint "  {FC3:MONOREPO_ROOT}  {555:-} {557:$MONOREPO_ROOT}"
cprint "  {FC3:OPENAI_API_KEY} {555:-} {557:$OPENAI_API_KEY}"

cprint
cprint "Aliases:"
cprint "  {FC3:gs}    {555:-} {557:alias for 'git status'}"
cprint "  {FC3:gcap}  {555:-} {557:alias for git add, commit, and push}"
cprint "  {FC3:gslog} {555:-} {557:recent git commits}"
cprint "  {FC3:scd}   {555:-} {557:change directory to closest match in repo}"




