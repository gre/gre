#!/bin/bash
rm -rf results/*.svg

for i in {0..255}; do
for samples in $(seq 2001 500 3001); do
for reversed in "" "--reversed"; do
    ./target/release/examples/wip --seed $i --samples $samples $reversed
    cp image.svg results/${i}_${samples}_${reversed}.svg
done
done
done