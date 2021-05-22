#!/bin/zsh

WIDTH=600
frames=8
FPS=$frames

set -e
cargo build --release --example wip
rm -rf results/*.svg results/out.* results/pngs || true
for i in $(seq 0 1 $(($frames - 1))); do
  ./target/release/examples/wip --frames $frames --index $i $*
    cp image.svg results/${i}.svg
done
cd results
mkdir pngs || true
for f in *.svg;
  do convert $f -resize ${WIDTH}x pngs/${f%.*}.png;
done
ffmpeg -r $frames -i pngs/%d.png -pix_fmt yuv420p -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" out.mp4

SPEED=1
palette="/tmp/palette.png"
filters="fps=$FPS,scale=$WIDTH:-1:flags=lanczos,setpts=$SPEED*PTS"
ffmpeg -v warning -i out.mp4 -vf "$filters,palettegen" -y $palette
ffmpeg -v warning -i out.mp4 -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y out.gif