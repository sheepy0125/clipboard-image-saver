# Clipboard image saver

# About

Ever had an image in your clipboard that you wanted to save?

Normally, one would need to open Paint or another image editing program, paste the image in, and save it.

With this amazing program, one would be able to save an image with a simple Ctrl+V!

# Features

-   [ ] Easy to use GUI that's mobile-first for some reason
-   [ ] Pasting images
-   [ ] Viewing pasted images
-   [ ] Saving images
-   [ ] Configuring auto paste or pasting manually
-   [ ] Configuring where images are saved
-   [ ] Configuring saving on paste or pressing a button to save
-   [ ] Transparency support
-   [ ] Converting to different file formats
-   [ ] Lightweight portable Linux application through an AppImage

# Building

## Setup

Note: These steps are specific for an Arch install with Systemd.
For other distros, you'll need to change some of these commands (e.g. `yay -S` -> `sudo apt-get install`)

### Rust

```sh
# See https://rustup.rs/.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### TailwindCSS

```sh
yay -S --needed docker --noconfirm  # Installs Docker. Note the `--noconfirm`, you may want to omit this
sudo systemctl enable --now docker  # Starts Docker on startup. To only run once, use `sudo systemctl start docker` instead
# sudo dockerd & disown             # If you're running another init system besides Systemd, try this
cd frontend/tailwind-yew-builder
docker-compose up dev               # Starts the TailwindCSS watcher
cd ../..
```

### Yew

```sh
cd frontend
# Yew needs the wasm-32-unknown-unknown target to work
rustup target add wasm32-unknown-unknown
cd ..
```

## Running

```sh
cargo tauri dev
# cargo tauri dev --release  # Improves performance (especially for reading the clipboard!), but slower compilation times
```

# License

This program is licensed under the MIT license. See `COPYING` for details.
