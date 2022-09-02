<div align="center">
    <h1> Tapy </h1>
    <img src="./assets/logo.png" alt="logo" width="256" height="256">
</div>

---

## Problem

Do you happen to have a huge collection of *favourited* (or *starred*, if you prefer) GIFs in Discord? Or maybe just hundreds of random photos in your Downloads directory?

Have you seen yourself in a situation where you need to quickly find a particular piece of media, let it be a GIF or a video, but just couldn't find it in your enormous arsenal?

Look no further - Tapy is here!

Tapy is a tool for tagging and quickly accessing all kinds of media. Just open it up, write a few keywords into the search box and voilà - everything is now filtered to only show media that matches your search tags.

Tapy consists of a backend and frontend server and can be consumed either through the provided frontend or directly through the `REST` API at `/api`.

## Building

Firstly, install the `sqlite3` library

```bash
# Debian/Ubuntu
sudo apt install sqlite3

# Arch
sudo pacman -S sqlite

# Nix
nix-env -iA nixpkgs.sqlite
```

- Add `wasm32-unknown-unknown` as a `rustc` target

```bash
rustup target add wasm32-unknown-unknown
```

- Install dependencies

```bash
cargo install trunk wasm-bindgen-cli
```

- Build frontend

```bash
pushd frontend
trunk build --release
popd
```

- Build backend

```bash
cargo build --release
```

## Running

```bash
cargo run
```

---

## TODO

- Frontend
    - (Working) Components for
        - viewing
            - [ ] one
            - [x] many
            items
        - adding
            - [ ] one
            - [ ] many
            items
- Backend
    - implement graphs for ranking most frequent tags
        - backend
        - frontend
