# Oneshot

Oneshot executes arbitrary scripts or starts a shell in a minimal temporary Podman container.

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

### Exec command

You can execute `oneshot` scripts using the exec command.

## 1shot scripts

A planned feature is being able to execute a `.1sh` script using a shebang.

```sh
#!/bin/oneshot
#!oneshot --image alpine:latest --from-uv
#!/usr/bin/env python3

import requests

response = requests.get('https://api.example.com')
print(response.json())
```
