#!/bin/sh

OUT=dist
NAME=grid

export RUSTFLAGS="--remap-path-prefix=$HOME=src"

wasm-pack build --target web --out-dir $OUT --out-name $NAME
wasm-opt -O3 -o $OUT/"$NAME"_bg.wasm $OUT/"$NAME"_bg.wasm
rm $OUT/package.json $OUT/"$NAME"_bg.d.ts $OUT/$NAME.d.ts $OUT/.gitignore $OUT/README.md
