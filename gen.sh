#!/bin/bash
for filename in ./image/*.png; do
    base=$(basename "$filename")
    name=${base%.*}
    out="./src/image/${name}.rs"
    w4 png2src --rust "$filename" --template image.mustache > "$out"
done