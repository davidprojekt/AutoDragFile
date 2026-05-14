# AutoDragFile

> :warning: Note: This tool only works under X11!

A fast, lightweight Rust utility that helps you programmatically initiate file drag-and-drop operations.

When executed, a small GTK window spawns directly beneath your cursor. You have 3 seconds to click and start dragging the file. Once you drop the file (or cancel the drag), the program automatically exits. If you don't start dragging within the 3-second window, it safely times out and closes out of your way.

The UI text is currently in German ("📎 Datei ziehen"), but you can easily tweak the text, window size, and timeout periods directly in src/main.rs!

## Usage
You can run it directly using Cargo:

```
cargo run --release -- /path/to/your/file
```

Alternatively, you can build the binary and run it directly:
```
# Build the executable
cargo build --release

# Run the compiled binary
./target/release/auto-drag-file /path/to/your/file
```

## Dependencies
If you intend to only use the binary and not compile it yourself, you will only need `xdotool`!

This tool requires the Rust toolchain (installed via `rustup` or your package manager), `xdotool`, and GTK 3 development headers to build.

> Note: `pkgconf` and `base-devel` are required to link the GTK libraries during the Rust build process.

#### Arch / Manjaro:
```
sudo pacman -S xdotool gtk3 pkgconf
```

#### Ubuntu / Debian:
```
sudo apt install xdotool libgtk-3-dev pkg-config build-essential
```