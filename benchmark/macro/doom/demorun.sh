#!/usr/bin/env sh

self=`realpath "$0"`
cur_dir=${self%/*}

crispy-doom -iwad "${cur_dir}"/DOOM.WAD -playdemo "${cur_dir}"/doomrun -strictdemo
