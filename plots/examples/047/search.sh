#!/bin/bash
rm -rf results/*.svg

pad=0.15
for i in $(seq 40 1 50); do
for samples in $(seq 200 200 800); do
for poly_threshold in $(seq 0.08 0.03 0.15); do
for f1 in $(seq 2 2 6); do
for f2 in $(seq 12 6 30); do
    echo ${i} ${pad} ${samples} ${poly_threshold} ${f1} ${f2}
    ./target/release/examples/wip ${i} ${pad} ${samples} ${poly_threshold} ${f1} ${f2}
    cp image.svg results/${i}_${pad}_${samples}_${poly_threshold}_${f1}_${f2}.svg
done
done
done
done
done