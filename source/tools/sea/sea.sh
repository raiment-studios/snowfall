#!/usr/bin/env bash
eval $MONOREPO_ROOT/source/tools/sea/sea $@ 2> $MONOREPO_ROOT/temp/__output.sh
cat $MONOREPO_ROOT/temp/__output.sh
source $MONOREPO_ROOT/temp/__output.sh
rm $MONOREPO_ROOT/temp/__output.sh