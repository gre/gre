#!/bin/bash
rm -rf results/*.svg

pad=0.15
poly_fill_samples=0
for i in $(seq 40 1 50); do
for samples in $(seq 100 100 500); do
for poly_threshold in $(seq 0.08 0.02 0.16); do
for f1 in $(seq 3 2 4); do
for f2 in $(seq 15 3 25); do
    echo ${i} ${pad} ${samples} ${poly_fill_samples} ${poly_threshold} ${f1} ${f2}
    ./target/release/examples/wip ${i} ${pad} ${samples} ${poly_fill_samples} ${poly_threshold} ${f1} ${f2}
    cp image.svg results/${i}_${pad}_${samples}_${poly_fill_samples}_${poly_threshold}_${f1}_${f2}.svg
done
done
done
done
done