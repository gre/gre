rm -rf results/*.svg

for i in $(seq 0 1 100); do
for noise_f in $(seq 7 2 7); do
for angular_speed in $(seq 0.04 0.02 0.04); do
    ./target/release/examples/wip $i $noise_f $angular_speed
    cp image.svg results/${i}_${noise_f}_${angular_speed}.svg
done
done
done