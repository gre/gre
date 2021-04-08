#!/bin/bash
rm -rf results/*.svg

for i in {0..255}; do
for power in $(seq 1 1 1); do
for divisor in $(seq 10 10 40); do
    ./target/release/examples/wip --seed $i --power $power --divisor $divisor
    cp image.svg results/${i}_${power}_${divisor}.svg
done
done
done