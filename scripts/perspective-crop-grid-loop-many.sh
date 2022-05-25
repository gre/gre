#!/bin/bash
SCRIPTS=`dirname $0`

# example
# ./scripts/perspective-crop-grid-loop.sh photo.jpg output_folder 8 640 640 4x2 569,1613 3251,1641 5962,1666 8702,1694 11529,1697 513,4303 3219,4347 5917,4390 8636,4434 11408,4478 375,7023 3130,7059 5861,7105 8597,7157 11366,7222

set -x
set -e
INPUT_IMAGES_FOLDER=$1
shift
INPUT=$1
shift
OUTPUT_DIR=$1
shift
FPS=$1
shift
WIDTH=$1
HEIGHT=$2
GRID=$3

# example of file
# 518 406,1147 3147,1196 5805,1238 8411,1270 11034,1259 297,3811 3118,3838 5806,3854 8424,3862 11029,3865 246,6563 3084,6524 5794,6509 8433,6487 11012,6473
# 519 353,1848 3214,1913 5977,1929 8762,1930 11588,1933 351,4648 3205,4663 5945,4684 8735,4708 11616,4732 371,7416 3185,7394 5916,7449 8715,7512 11610,7587

if [ ! -f "$INPUT" ] || [ ! -d "$INPUT_IMAGES_FOLDER" ] || [ -z "$OUTPUT_DIR" ] || [ -z "$FPS" ] || [ -z "$GRID" ] ; then
  echo "Usage: $0 folder listfile.txt outputdir fps width height 4x2"
  exit
fi

mkdir -p $OUTPUT_DIR


IFS=$'\n'
I=0
cat $INPUT | while read line; do
  file=`echo $line | cut -d ' ' -f 1`
  rest=`echo $line | cut -d ' ' -f 2-`
  echo $rest | xargs ./scripts/perspective-crop-grid-loop.sh $INPUT_IMAGES_FOLDER/$file.jpg $OUTPUT_DIR $FPS $WIDTH $HEIGHT $GRID
  mv $OUTPUT_DIR/out.gif $OUTPUT_DIR/${file}.gif
  mv $OUTPUT_DIR/out.mp4 $OUTPUT_DIR/${file}.mp4
done