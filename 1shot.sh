
#!/bin/bash
set -e  # Exit on any error
trap 'echo "An error occurred. Exiting..."; exit 1' ERR

IMAGE="${ONESHOT_IMAGE:-alpine:latest}"

show_help() {
    echo "Usage: 1shot [-i] [-s 'script'] [command]"
    echo "  -i          : Start interactive mode"
    echo "  -s 'script' : Execute the provided script"
    echo "  help        : Show this help message"
    echo "If no arguments are provided, this help message is shown."
}

podman machine init 2>&1 | grep -v "VM already exists" || true
podman machine start 2>&1 | grep -v "already running" || true

case "$1" in
    -i)
        podman run -it --rm \
            -v "$HOME:$HOME" \
            -w "$HOME" \
            -e "PS1=\[\033[1;32m\]1shot\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ " \
            "$IMAGE" /bin/sh
        ;;
    -s)
        if [ -z "$2" ]; then
            echo "Error: No script provided with -s option"
            show_help
            exit 1
        fi
        podman run -i --rm \
            -v "$HOME:$HOME" \
            -w "$HOME" \
            "$IMAGE" /bin/sh -c "$2"
        ;;
    help|"")
        show_help
        ;;
esac

