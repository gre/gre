Similarly to "One day, One shader" ( https://github.com/gre/shaderday.com ), this [Rust](https://www.rust-lang.org/) project is a "generative art" that, for each entry in `examples/`, generates a SVG file and plot it with an AxiDraw.

**I also share them on https://twitter.com/greweb/status/1344741218962563072?s=19**

### January 2021

<a href="examples/001"><img height="200" src="./examples/001/photo.jpg"/></a>
<a href="examples/002"><img height="200" src="./examples/002/IMG_20210102_102357.jpg"/></a>
<a href="examples/003"><img height="200" src="./examples/003/IMG_20210103_084109.jpg"/></a>
<a href="examples/004"><img height="200" src="./examples/004/image.jpg"/></a>
<a href="examples/005"><img height="200" src="./examples/005/IMG_20210104_212108__01.jpg"/></a>
<a href="examples/006"><img height="200" src="./examples/006/photo.png"/></a>
<a href="examples/007"><img height="200" src="./examples/007/photo.jpg"/></a>
<a href="examples/008"><img height="200" src="./examples/008/photo.jpg"/></a>
<a href="examples/009"><img height="200" src="./examples/009/photo_y1.jpg"/></a>
<a href="examples/010"><img height="200" src="./examples/010/photo.jpg"/></a>
<a href="examples/011"><img height="200" src="./examples/011/photo.jpg"/></a>
<a href="examples/012"><img height="200" src="./examples/012/photo.jpg"/></a>
<a href="examples/013"><img height="200" src="./examples/013/photo.jpg"/></a>

---

(replace 000 with day number)

```
cargo run --example 000
```

How to do "hot reload":

Run this

```
cargo watch "run --example 000"
```

And then open the `image.svg` with a viewer that allows to update when the file changes. (E.g. vscode SVG Viewer)
