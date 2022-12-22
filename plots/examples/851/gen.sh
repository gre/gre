#!/bin/bash

seed=${1:-0}
set -e
mkdir -p results
mkdir -p results/videos
mkdir -p results/svgs
mkdir -p results/tmp
i=0

echo "GENERATE SVGs for seed ${seed}"
for page in $(seq 0 1 1); do
    cargo run --release --example 851 -- --seed $seed --page $page --file results/svgs/seed_${seed}_${page}.svg &
done
wait

cd results

echo "GENERATE PNGs for seed ${seed}"
for page in $(seq 0 1 1); do
    convert svgs/seed_${seed}_${page}.svg tmp/seed_${seed}_page_${page}.png &
done
wait

echo "CROP PNGs for seed ${seed}"
for page in $(seq 0 1 9); do
    for xi in $(seq 0 1 1); do
        for yi in $(seq 0 1 3); do
            w=$((1123/2))
            h=$((1587/4))
            x=$(($xi*$w))
            y=$(($yi*$h))
            convert tmp/seed_${seed}_page_${page}.png -crop ${w}x${h}+${x}+${y} tmp/seed_${seed}_$i.png &
            i=$(($i+1))
        done
    done
done
wait

echo "Generate video for seed ${seed}"
ffmpeg -y -r 8 -i tmp/seed_${seed}_%d.png -c:v libx264 -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2,fps=8,format=yuv420p" videos/$seed.mp4

rm tmp/seed_${seed}_*.png #cleanup

echo "DONE for seed ${seed}"
