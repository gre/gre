

rm -rf dist
mkdir dist
cat $PWD/index.mjs $PWD/main.mjs | uglifyjs -c -m -e 1> dist/index.mjs
cat  $PWD/features.pre.js $PWD/index.mjs $PWD/features.js  $PWD/features.post.js | uglifyjs -c -m 1> dist/features.js
echo "module.exports=" > dist/features.node.js
cat dist/features.js >> dist/features.node.js
ls -l dist
cp index.html dist/
