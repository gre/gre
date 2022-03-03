rm -rf video-*/
node node-anim.js $1
video=`ls | grep video-`
ffmpeg -y -framerate 24 -i $video/frame%08d.jpg -vf "vflip" -c:v libx264 -pix_fmt yuv420p -r 24 out.mp4
rm -rf video-*/
ffmpeg -i out.mp4 -vf 'scale=2000:2000' $1.mp4
rm out.mp4