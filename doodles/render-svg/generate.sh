RES=out

. ~/.livedraw/render-svg-envs

inkscape -d $SVG_DPI $SVG_INPUT -o $IMAGE_INPUT

set -e
mkdir -p results
rm -rf video-*/ results/$RES.mp4 results/$RES.jpg
node ./node.mjs
video=`ls | grep video-`
if [ $FRAMES -ne 1 ]; then
    ffmpeg -y -framerate $FRAMERATE -i $video/frame%08d.jpg -loglevel error -vf scale=$WIDTH:$HEIGHT -c:v libx264 -pix_fmt yuv420p -r $FRAMERATE out.mp4
    mv out.mp4 results/$RES.mp4
fi
convert -resize ${WIDTH}x${HEIGHT} video-*/frame00000000.jpg -quality 90% results/$RES.jpg
rm -rf video-*/
