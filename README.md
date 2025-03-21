# Oneshot

Oneshot executes arbitrary scripts or starts a shell in a minimal temporary Podman container. It is designed to be an ergonomic interface for creating one-off scripts inside of short-lived temporary containers. 

## Usage

### Interactive shell

```sh
oneshot shell
```

### Run command

#### Download a YouTube video

This script installs `yt-dlp` using `uv` and runs it.

```sh
oneshot run -s "yt-dlp dQw4w9WgXcQ" --from-uv yt-dlp
```

## Development

You can develop within a devcontainer that includes Rust, Homebrew, NeoVim,
Docker-outside-of-docker, and Podman features.

```sh
devpod up .
devpod ssh .
```

Then just use cargo to interact with the application. Arguments after the `--`
are read by the app.

```sh
cargo run -- --help
```

Example

```sh
cargo run -- run -s "yt-dlp dQw4w9WgXcQ" --from-uv yt-dlp
```
