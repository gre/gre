

rm -rf dist
mkdir dist
cat $PWD/index.mjs $PWD/engine/main.mjs | uglifyjs -d "FEATURE_MODE=false" -c -m -e 1> dist/index.mjs
cat $PWD/engine/features.pre.js $PWD/index.mjs $PWD/engine/features.js  $PWD/engine/features.post.js | uglifyjs -d "FEATURE_MODE=true" -c -m 1> dist/features.js
cat dist/features.js  $PWD/engine/genfeatures.js 1> dist/genfeatures.js
echo "module.exports=" > dist/features.node.js
cat dist/features.js >> dist/features.node.js
ls -l dist
cp index.html dist/
