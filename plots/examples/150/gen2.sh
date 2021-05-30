#!/bin/zsh

set -e

P=$(dirname $0)
rm -rf out/*.gif || true
mkdir -p out


for i in $(seq 0 1 100); do
  sh $P/gen.sh --seed $i
  cp ./results/out.gif out/${i}.gif
done

# for a in 0.5 1 2; do
# for b in 0; do
# for c in 0.5 1 2; do
# for d in 2 3 4; do
  # sh $P/gen.sh --a $a --b $b --c $c --d $d
  # cp ./results/out.gif out/${a}_${b}_${c}_${d}.gif
# done
# done
# done
# done