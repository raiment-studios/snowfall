#!/usr/bin/env bash
eval $MONOREPO_ROOT/source/tools/sea/sea $@ 2> $MONOREPO_ROOT/temp/__sea_output.sh
source $MONOREPO_ROOT/temp/__sea_output.sh