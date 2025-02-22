#!/bin/bash
IMAGE="${ONESHOT_IMAGE:-alpine:latest}"

podman machine init 2>&1 | grep -v "VM already exists" || true
podman machine start 2>&1 | grep -v "already running" || true

if [ $# -eq 0 ]; then
    # No arguments, start interactive shell
    podman run -it --rm \
        -v "$HOME:$HOME" \
        -w "$HOME" \
        -e "PS1=\[\033[1;32m\]1shot\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ " \
        $IMAGE /bin/sh
else
    # Arguments provided, treat as bash script
    podman run -i --rm \
        -v "$HOME:$HOME" \
        -w "$HOME" \
        $IMAGE /bin/sh -c "$*"
fi

set -e  # Exit on any error
trap 'echo "An error occurred. Exiting..."; exit 1' ERR
