# ⚶ Vesta

Vesta is a native desktop super-app in Rust, built with [egui/eframe](https://github.com/emilk/egui) for Windows and Linux.

> Early development: the app does not open a window yet. Progress is tracked in the Linear project "⚶ Vesta".

## Requirements

Vesta runs on Windows and Linux only; building it for any other system fails with a clear message.

On Linux, Vesta needs a Wayland or X11 desktop session and these libraries, which most desktop distributions already provide:

- a graphics driver with OpenGL or OpenGL ES support (Mesa or the vendor's), which provides `libEGL` and `libGL`;
- `libxkbcommon`;
- on Wayland: `libwayland-client`, `libwayland-cursor` and `libwayland-egl`;
- on X11: `libX11`, `libX11-xcb`, `libXcursor`, `libXi`, `libxcb` and `libxkbcommon-x11`.

## Build

Requires Rust 1.95 (pinned by `rust-toolchain.toml`, installed automatically by rustup).

```sh
cargo run              # build and run
cargo test             # run the tests
cargo doc --no-deps --open   # build and open the API documentation
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the workflow, the checks to run before pushing, and the lint policy.
