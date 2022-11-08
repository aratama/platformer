#!/bin/bash
for filename in ./image/*.png; do
    base=$(basename "$filename")
    name=${base%.*}
    out="./src/image/${name}.rs"
    w4 png2src --rust "$filename" > "$out"
    up=${name^^}
    echo "use crate::image::Image;" >> $out
    echo "pub const ${up}_IMAGE: Image = Image {" >> $out
    echo "    width: ${up}_WIDTH," >> $out
    echo "    height: ${up}_HEIGHT," >> $out
    echo "    flags: ${up}_FLAGS," >> $out
    echo "    data: &${up}," >> $out
    echo "};" >> $out
done