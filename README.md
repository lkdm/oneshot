# Podshot

Podshot executes arbitrary scripts or starts a shell in a minimal temporary Podman container. This allows arbitrary execution of
scripts or programs without installing dependencies to the host.


## Usage

### Download a YouTube video

This script installs `yt-dlp` using `uv` and runs it.

```sh
podshot run "yt-dlp dQw4w9WgXcQ" --from-uv yt-dlp
```
