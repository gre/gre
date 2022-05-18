#!/bin/bash
SCRIPTS=`dirname $0`

# example
# ./scripts/perspective-crop-grid-loop.sh photo.jpg output_folder 8 640 640 4x2 569,1613 3251,1641 5962,1666 8702,1694 11529,1697 513,4303 3219,4347 5917,4390 8636,4434 11408,4478 375,7023 3130,7059 5861,7105 8597,7157 11366,7222

set -x
set -e
INPUT=$1
shift
OUTPUT_DIR=$1
shift
FPS=$1
shift
WIDTH=$1
HEIGHT=$2

if [ -z "$INPUT" ] || [ -z "$OUTPUT_DIR" ] || [ -z "$FPS" ]; then
  echo "Usage: $0 input outputdir fps width height 4x2 ...list_of_pixel_positions..."
  exit
fi

mkdir -p $OUTPUT_DIR


IFS=$'\n'
I=0
for param in `node $SCRIPTS/perspective-crop-grid-loop-make-params.js $*`; do
    convert $INPUT -matte -virtual-pixel transparent -distort Perspective $param -crop ${WIDTH}x${HEIGHT}+0+0 $OUTPUT_DIR/$I.png
    I=$(($I+1))
done

ffmpeg -y -r 8 -i $OUTPUT_DIR/%d.png -c:v libx264 -vf "fps=$FPS,format=yuv420p" $OUTPUT_DIR/out.mp4
palette=$OUTPUT_DIR/palette.png
filters="fps=$FPS,scale=$WIDTH:$HEIGHT:flags=lanczos"
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -vf "$filters,palettegen" -y $palette
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $OUTPUT_DIR/out.gif
