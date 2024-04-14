# ld55 - merge or die

This is a ludum dare combo entry on the theme "Summoning".

- [Hackathon entry page](https://ldjam.com/events/ludum-dare/55/merge-or-die)
- [Play the game on Github](https://josepedrodias.github.io/ld55/dist/)


## reference
- https://docs.rs/comfy/latest/comfy/
- https://comfyengine.org/book/releasing/
- https://github.com/darthdeus/comfy/tree/master/comfy/examples

## build

    cargo run

    cargo build --release --features comfy/ci-release

## build for web

    trunk serve --open

    trunk build --release --features comfy/ci-release


## other

- https://rustwasm.github.io/wasm-bindgen/api/web_sys/
- https://rustwasm.github.io/wasm-bindgen/examples/dom.html
