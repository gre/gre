Similarly to "One day, One shader", this [Rust](https://www.rust-lang.org/) project is a "generative art" that, for each entry in `examples/`, generates a SVG file and plot it with an AxiDraw.

**I also share them on https://twitter.com/greweb/status/1344741218962563072?s=19**

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
