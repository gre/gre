set -e
WD="generating-${2:-0}"
mkdir -p $WD
cd $WD
mkdir -p ../results
rm -rf video-*/
node ../node.mjs $1
video=`ls | grep video-`
ffmpeg -y -framerate 30 -i $video/frame%08d.jpg -loglevel error -vf scale=1920:1920 -c:v libx264 -pix_fmt yuv420p -r 30 out.mp4
mv out.mp4 ../results/$1.mp4
cp video-*/frame00000030.jpg ../results/$1.jpg
cp video-*/frame00000000.jpg ../results/$1-alt.jpg
mv metadata.json ../results/$1
rm -rf video-*/

