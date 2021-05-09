#!/bin/zsh

cargo build --release --example wip
rm -rf results/*.svg results/out.mp4 results/pngs
count=8
for j in $(seq 0 1 $(($count - 1))); do
  i=$((($j + 3) % 8))
  f=$(((0.5 + $i) / $count.));
  echo $i $f
  ./target/release/examples/wip --f $f --technique $i
    cp image.svg results/${j}.svg
done
cd results
mkdir pngs
for f in *.svg;
  do convert $f pngs/${f%.*}.png;
done
ffmpeg -r $count -i pngs/%d.png -pix_fmt yuv420p -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" out.mp4