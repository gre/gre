#!/bin/bash
rm -rf results/*.svg

pad=0.15
for i in $(seq 50 1 100); do
for samples in $(seq 800 300 1600); do
for poly_threshold in $(seq 0.03 0.02 0.06); do
for f1 in $(seq 3 1 6); do
for f2 in $(seq 15 5 30); do
    echo ${i} ${pad} ${samples} ${poly_threshold} ${f1} ${f2}
    ./target/release/examples/wip ${i} ${pad} ${samples} ${poly_threshold} ${f1} ${f2}
    cp image.svg results/${i}_${pad}_${samples}_${poly_threshold}_${f1}_${f2}.svg
done
done
done
done
done