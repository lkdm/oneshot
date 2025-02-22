
#!/bin/bash
set -e  # Exit on any error
trap 'echo "An error occurred. Exiting..."; exit 1' ERR

IMAGE="alpine:latest"  # Set default image


show_help() {
    echo "Usage: 1shot <command> [options]

Commands:
  run <command>        Run a single command in a container
  shell                Start an interactive shell in a container
  help                 Show this help message

Options:
  -i                   Specify a target image, default=alpine:latest

Podman must be installed.
  "
}

init() {
	if [[ "$1" == "-i" || "$1" == "--image" ]]; then
    	if [[ -n "$2" ]]; then
        	IMAGE="$2"
        	shift 2
    	else
        	echo "No image specified after -i/--image flag. Using default: alpine:latest"
        	shift 1
    	fi
	fi

	if [[ "$1" == "-i" || "$1" == "--image" ]]; then
    	IMAGE="$2"
    	shift 2
	fi

	podman machine init 2>&1 | grep -v "VM already exists" || true
	podman machine start 2>&1 | grep -v "already running" || true
}

case "$1" in
    shell)
    	init
        podman run -it --rm \
            -v "$HOME:$HOME" \
            -w "$HOME" \
            -e "PS1=\[\033[1;32m\]1shot\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ " \
            "$IMAGE" /bin/sh
        ;;
    run)
        if [ -z "$2" ]; then
            echo "Error: No script provided with -s option"
            show_help
            exit 1
        fi
        init
        podman run -i --rm \
            -v "$HOME:$HOME" \
            -w "$HOME" \
            "$IMAGE" /bin/sh -c "$2"
        ;;
    help|"")
        show_help
        ;;
esac

