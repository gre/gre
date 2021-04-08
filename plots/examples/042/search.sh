#!/bin/bash
rm -rf results/*.svg

colors=crimson,black,crimson
bg=white
rot=0.5
small=10
big=90
length_mul=2.2

for i in 26; do
for size in $(seq 100 50 300); do
for length in $(seq 40 20 80); do
for force in $(seq 1.2 0.2 1.2); do
for freq_mul in $(seq 0.6 0.2 0.6); do
    ./target/release/examples/unreleased_squares $i $size $force $length $colors $bg $freq_mul $rot $small $big $length_mul
    cp image.svg results/${i}_${size}_${force}_${length}_${colors}_${freq_mul}_${rot}_${small}_${big}_${length_mul}.svg
done
done
done
done
done