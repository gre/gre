#!/bin/bash
SCRIPTS=`dirname $0`

# example
# ./scripts/perspective-crop-grid-loop.sh folder output_folder 8 2x4

set -x
set -e
INPUT=$1
shift
OUTPUT_DIR=$1
shift
FPS=$1
shift
WIDTH=${WIDTH:-1280}
HEIGHT=${HEIGHT:-906}

if [ -z "$INPUT" ] || [ -z "$OUTPUT_DIR" ] || [ -z "$FPS" ]; then
  echo "Usage: $0 input outputdir fps width height 4x2 ...list_of_pixel_positions..."
  exit
fi

mkdir -p $OUTPUT_DIR


IFS=$'\n'

P=0
I=0
cat $INPUT/list.txt | while read line; do
  if [ -f $INPUT/${P}.jpg ]; then
    for param in `echo $line | xargs node $SCRIPTS/perspective-crop-grid-loop-make-params.js $WIDTH $HEIGHT $*`; do
        convert $INPUT/${P}.jpg -matte -virtual-pixel transparent -distort Perspective $param -crop ${WIDTH}x${HEIGHT}+0+0 $OUTPUT_DIR/$I.png
        I=$(($I+1))
    done
  fi
  P=$(($P+1))
done

w=$WIDTH
h=$HEIGHT

ffmpeg -y -r $FPS -i $OUTPUT_DIR/%d.png -c:v libx264 -vf "fps=$FPS,format=yuv420p" $OUTPUT_DIR/out.mp4
palette=$OUTPUT_DIR/palette.png
filters="fps=$FPS,scale=$WIDTH:$HEIGHT:flags=lanczos"
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -vf "$filters,palettegen" -y $palette
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $OUTPUT_DIR/out-full.gif
WIDTH=640
HEIGHT=$(($WIDTH*$h/$w))
filters="fps=$FPS,scale=$WIDTH:$HEIGHT:flags=lanczos"
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -vf "$filters,palettegen" -y $palette
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $OUTPUT_DIR/out-640.gif
WIDTH=320
HEIGHT=$(($WIDTH*$h/$w))
filters="fps=$FPS,scale=$WIDTH:$HEIGHT:flags=lanczos"
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -vf "$filters,palettegen" -y $palette
ffmpeg -v warning -i $OUTPUT_DIR/out.mp4 -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $OUTPUT_DIR/out-320.gif
