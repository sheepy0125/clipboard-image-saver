# Clipboard image saver

# About

Ever had an image in your clipboard that you wanted to save?

Normally, one would need to open Paint or another image editing program, paste the image in, and save it.

With this amazing program, one would be able to save an image with a simple button click!

# Features

-   [x] Easy to use GUI that's mobile-first for some reason
-   [x] Pasting images
-   [x] Viewing pasted images
-   [x] Dragging and zooming pasted images
-   [x] Saving images
-   [x] Configuring auto paste or pasting manually
-   [x] Configuring where images are saved
-   [x] Configuring anti-aliasing
-   [x] Configuring saving as different file formats
-   [x] Transparency support
-   [x] Lightweight portable Linux application through an AppImage

# Building

## Setup

Note: These steps are specific for an Arch install with Systemd.
For other distros, you'll need to change some of these commands (e.g. `yay -S` -> `sudo apt-get install` and `yay -R` -> `sudo apt-get remove`)

### Rust

```sh
# See https://rustup.rs/.
yay -R rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup override set nightly
source $HOME/.cargo/env
```

### TailwindCSS

```sh
# In the project root
yay -S --needed docker docker-compose # Installs Docker
sudo systemctl enable --now docker    # Starts Docker on startup. To only run once, use `sudo systemctl start docker` instead
# sudo dockerd & disown               # If you're running another init system besides Systemd, try this
cd frontend/tailwind-yew-builder
sudo docker-compose up dev            # Starts the TailwindCSS watcher
cd ../..
```

### Yew

```sh
# In the project root
cd frontend
rustup target add wasm32-unknown-unknown
cd ..
```

### Tauri

```sh
# In the project root
cargo install tauri-cli
```

## Checking code

```sh
# In the project root
./check.sh
```

## Running

```sh
# In the project root
cargo tauri dev --release
```

## Building

```sh
# In the project root
./build.sh
```

# License

This program is licensed under the MIT license. See `COPYING` for details.
Chicken
