
#!/bin/bash

colors=white,gold,orange
rm -rf results/*.svg
for i in {0..100}; do
for size in $(seq 100 25 100); do 
for length in $(seq 60 20 60); do
for force in $(seq 0.8 0.2 1.0); do
    ./target/release/examples/wip $i $size $force $length $colors
    cp image.svg results/${i}_${size}_${force}_${length}_${colors}.svg
done
done
done
done