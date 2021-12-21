set -e
START=${1:-0}
LAST=${2:-2047}
for i in $(seq $START 1 $LAST); do
  echo $i
  ./generate-one.sh $i $RANDOM
done
