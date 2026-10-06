# WebBrot

An in-browser, GPU-parallel Mandelbrot fractal generator, based on
[mbrot](https://github.com/Logan-010/mbrot).

## Running

With Rust's `wasm32-unknown-unknown` target and [Trunk](https://trunkrs.dev/) installed:

```sh
trunk serve
```

Open the URL printed by Trunk in a browser with WebGL2 and hardware acceleration enabled.
Use `trunk build --release` for an optimized build.
