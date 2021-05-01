cargo build --release --example wip
rm -rf results
mkdir results
for i in {0..15}; do
  ./target/release/examples/wip --frame $i;
  cp image.svg results/$i.svg;
done;
cd results
mkdir pngs
for f in *.svg; do
  convert $f pngs/${f%.*}.png;
done
ffmpeg -r 8 -i pngs/%d.png -pix_fmt yuv420p -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" raw.mp4
ffmpeg -i raw.mp4 -filter_complex "[0:v]reverse,fifo[r];[0:v][r] concat=n=2:v=1 [v]" -map "[v]" output.mp4
cd -