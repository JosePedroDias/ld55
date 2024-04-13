# ld55

## mechanics

based in seaside escape ads

- 2 player mah jongg themed tiles
- setup with 7*4*2 tiles, where there are 14 different tile faces, 7 of each theme (ex: emojis + flags)
- shuffle all tiles
- give each player 7*4 tiles facing down



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

# TODO

- use get_time
- use lerp
- use persistence?
