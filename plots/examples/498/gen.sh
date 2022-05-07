#!/bin/bash

set +e
cd results
convert ../image.svg image.png

# 37 37
# 350 506
convert image.png -crop 350x506+37+37 0.png
convert image.png -crop 350x506+424+37 1.png
convert image.png -crop 350x506+811+37 2.png
convert image.png -crop 350x506+1198+37 3.png
convert image.png -crop 350x506+37+580 4.png
convert image.png -crop 350x506+424+580 5.png
convert image.png -crop 350x506+811+580 6.png
convert image.png -crop 350x506+1198+580 7.png
ffmpeg -y -r 8 -i %d.png -c:v libx264 -vf "fps=8,format=yuv420p" out.mp4