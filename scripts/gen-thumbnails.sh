
cd public/images/plots
for f in *.jpg *.gif; do
  if [ ! -f ${f%%.*}-thumbnail.jpg ]; then
    if [[ $f =~ ^[0-9]+\.jpg$ ]]; then
      convert $f -resize 512x512\> ${f%%.*}-thumbnail.jpg;
    fi
    if [[ $f =~ ^[0-9]+\.gif$ ]]; then
      convert $f[0] -resize 512x512\> ${f%%.*}-thumbnail.jpg;
    fi
  fi
done