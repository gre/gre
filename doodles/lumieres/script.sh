rm -rf video-*/
node node.js $1
video=`ls | grep video-`
ffmpeg -y -framerate 24 -i $video/frame%08d.jpg -vf "vflip" -c:v libx264 -pix_fmt yuv420p -r 24 out.mp4
rm -rf video-*/
mv out.mp4 $1.mp4

