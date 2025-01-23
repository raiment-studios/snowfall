
[private]
_common-default:
  @just --list --unsorted

_common-ensure-webapp:
    cp -f $MONOREPO_ROOT/source/common/Makefile.common Makefile
    cp -Rf $MONOREPO_ROOT/.vscode/ .vscode/
    cp -f $MONOREPO_ROOT/config/mprocs.yaml mprocs.yaml
    @which mprocs > /dev/null
    @which deployctl > /dev/null
    npm install



_common-dev-watch directory command:
    npx nodemon \
        --watch {{directory}} \
        --ext ts,tsx,html,css,json,yaml,yml,txt \
        --exec "{{command}}"

_common-build-bundle source target:
    echo "Running build-bundle for {{source}} -> {{target}}"
    npx esbuild \
        --preserve-symlinks \
        --loader:.js=jsx \
        --loader:.md=text \
        --loader:.yaml=text \
        --loader:.txt=text \
        --sourcemap \
        --bundle {{source}} \
        --outfile={{target}}
    echo "Done running build-bundle for {{source}} -> {{target}}"