set -e

cd files
while true; do
    id=`curl -s http://localhost:3000/task/ffmpeg`
    if [[ "$id" != "" ]]; then
        echo $id
        cd $id
        ffmpeg -y -framerate 30 -i frame%08d.png -loglevel error -vf scale=1920:1920 -c:v libx264 -pix_fmt yuv420p -r 30 out.mp4
        mv out.mp4 video.mp4
        cp frame00000030.png image.png
        cp frame00000000.png image-alt.png
        rm -f frame*.png out.mp4
        cd -
    else
        echo "nothing to do..."
        sleep 10
    fi
done