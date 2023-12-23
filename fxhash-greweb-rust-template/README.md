### License

This template is used by @greweb to create most of his "Plottable" art works. 
Because it contains specific code used by him, it is not intended to be used by anyone else.
That said, if you still want to use it, you must mention in your work that this template is used (Attribution) and you can't change the license (ShareAlike), accordingly to CC BY-SA 4.0.

https://creativecommons.org/licenses/by-sa/4.0/

### Usage

**prerequisites:**

```
# Node.JS setup
# https://nodejs.org/en/download/package-manager/

# Rust+WASM setup
# https://rustup.rs/
# https://rustwasm.github.io/wasm-pack/installer/
```

**first start:**

First of all, install deps and starts a local server, hosting the dist folder

```
npm i
npm run server
```

Then, chose one of these:

**1 – build for production:**

```
npm run build
```

**2 – run during development:**

```
npm start
```

**3 – debug on the web:** *(slower WASM execution but debuggable)*

```
BUILD_MODE=development npm start
```