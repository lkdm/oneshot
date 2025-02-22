#!/bin/bash
podman machine init 2>&1 | grep -v "VM already exists" || true
podman machine start 2>&1 | grep -v "already running" || true
podman run -it --rm \
	-v "$HOME:$HOME" \
	-w "$HOME" \
	-e "PS1=\[\033[1;32m\]1shot\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ " \
	alpine:latest /bin/sh -c "${1:-/bin/sh}"

