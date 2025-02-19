
[private]
_common-default:
  @just --list --unsorted


cprintln := "$MONOREPO_ROOT/source/tools/sea/sea cprintln"

_common-ensure-webapp:
    mkdir -p dist
    @just _ensure-line-in-file ".gitignore" "mprocs.yaml"
    @just _ensure-line-in-file ".gitignore" ".vscode/"
    @just _ensure-line-in-file ".gitignore" "node_modules/"
    @just _ensure-line-in-file ".gitignore" "dist/"
    @just _cprint "Copying VSCode settings..."
    @cp -Rf $MONOREPO_ROOT/.vscode/ .vscode/    
    @just __copy-with-preamble "source/common/Makefile.common" "Makefile"    
    @just __copy-with-preamble "config/mprocs.yaml" "mprocs.yaml"
    @which mprocs > /dev/null
    @which deployctl > /dev/null
    @just _cprint "Running npm install..."
    @npm install > /dev/null || npm install



_common-dev-watch directory command:
    npx nodemon \
        --watch {{directory}} \
        --ext ts,tsx,html,css,json,yaml,yml,txt,jpg,png \
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


_common-prepend-to-file file content:

_cprint message:
    @"$MONOREPO_ROOT/source/tools/sea/sea" "cprintln" "{35a:{{message}}}"



__copy-with-preamble source target:
    @just _cprint "Copying {{source}}.."
    @echo "{{trim(generated_file_preamble)}}" > {{target}}
    @echo "#" >> {{target}}
    @echo "# The source file is located at:" >> {{target}}
    @echo "# {{source}}" >> {{target}}
    @echo "#" >> {{target}}
    @echo "############################################################" >> {{target}}
    @echo "" >> {{target}}
    @cat $MONOREPO_ROOT/{{source}} >> {{target}}

_link-src-lib target:
    @sea cprintln "Ensuring symbolic link for {DA5:{{target}}}"
    @just _ensure-line-in-file ".gitignore" "src/{{target}}"
    @just _make-link-src-lib {{target}}

_make-link-src-lib target:
    #!/usr/bin/env bash
    TARGET=./src/{{target}}
    SOURCE=$MONOREPO_ROOT/source/lib/{{target}}/src    
    if [ ! -L $TARGET ] && [ -e $SOURCE ]; then
        ln -s $SOURCE $TARGET
    fi

_ensure-line-in-file file line:
    #!/usr/bin/env bash
    # Check if the file does not end with a newline
    if [ -n "$(tail -c 1 "{{file}}")" ]; then
        echo "" >> "{{file}}"
    fi
    if ! grep -Fxq "{{line}}" "{{file}}"; then
        echo "{{line}}" >> "{{file}}"
    fi


generated_file_preamble := '''
    ############################################################
    # GENERATED FILE: DO NOT EDIT
    ############################################################
'''