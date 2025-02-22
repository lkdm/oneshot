#!/bin/bash
set -e  # Exit on any error
trap 'echo "An error occurred. Exiting..."; exit 1' ERR

show_help() {
    echo "Usage: 1shot <command> [options]

Commands:
  run <command>        Run a single command in a container
  shell                Start an interactive shell in a container
  help                 Show this help message

Options:
  -i, --image <image>  Specify a target image, default=alpine:latest
  -o, --output-dir <path>    Specify the output directory (default: current directory)
  --from-apk   [pkgs]  Install packages from apk
  --from-git   [repos] Download repositories from git
  --from-cargo [pkgs]  Install packages from cargo
  --from-uv    [pkgs]  Install packages from uv

Examples:

  Run an interactive shell:
  	1shot shell

  Download a YouTube video:
    1shot run 'yt-dlp dQw4w9WgXcQ' --from-uv yt-dlp

  Convert an image format:
	1shot run 'convert input.png output.jpg' --from-apk imagemagick

  Perform a quick network scan:
	1shot run 'nmap -p 80,443 example.com' --from-apk nmap

  Pretty print JSON:
  	echo '{\"foo\":\"bar\", \"baz\":[1,2,3]}' | 1shot run 'jq .' --from-apk jq


Podman must be installed.
"
}

IMAGE="alpine:latest"
INSTALL_COMMANDS=""
COMMAND=""
OUTPUT_DIR="$(pwd)"

parse_args() {
	if [[ "$1" == "run" ]]; then
        shift
        RUN_COMMAND="$1"
        shift
    fi

    while [[ "$#" -gt 0 ]]; do
        case $1 in
            -i|--image)
                IMAGE="$2"
                shift 2
                ;;
            -o|--output-dir)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            --from-apk)
                if [[ ! $INSTALL_COMMANDS =~ "apk update" ]]; then
                    INSTALL_COMMANDS+="apk update && "
                fi
                if [ -n "$2" ] && [[ "$2" != --* ]]; then
                    INSTALL_COMMANDS+="apk add --no-cache $2 && "
                    shift
                else
                    INSTALL_COMMANDS+="apk add --no-cache && "
                fi
                shift
                ;;
            --from-git)
                INSTALL_COMMANDS+="apk add --no-cache git && "
                if [ -n "$2" ] && [[ "$2" != --* ]]; then
                    INSTALL_COMMANDS+="git clone $2 && "
                    shift
                fi
                shift
                ;;
            --from-cargo)
                INSTALL_COMMANDS+="apk add --no-cache cargo && "
                if [ -n "$2" ] && [[ "$2" != --* ]]; then
                    INSTALL_COMMANDS+="cargo install $2 && "
                    shift
                fi
                shift
                ;;

			--from-uv)
    			INSTALL_COMMANDS+="apk add --no-cache python3 py3-pip && \
    			python3 -m venv /app/venv && \
    			. /app/venv/bin/activate && \
    			pip install uv && \
    			"
    			UV_PACKAGES=""
    			while [ -n "$2" ] && [[ "$2" != -* ]]; do
        			UV_PACKAGES+="$2 "
        			shift
    			done
    			if [ -n "$UV_PACKAGES" ]; then
        			INSTALL_COMMANDS+="uv pip install $UV_PACKAGES && "
    			fi
    			INSTALL_COMMANDS+="deactivate && . /app/venv/bin/activate &&"
    			shift
    			;;
            *)
                COMMAND="$@"
                break
                ;;
        esac
    done
}

OUTPUT_DIR="$(realpath "$OUTPUT_DIR")"

init() {
    podman machine init 2>&1 | grep -v "VM already exists" || true
    podman machine start 2>&1 | grep -v "already running" || true
}

COMMAND_TYPE="${1}"
shift

parse_args "$COMMAND_TYPE" "$@"

case "$COMMAND_TYPE" in
    run)
        podman run -i --rm \
            -v "$OUTPUT_DIR:/OUTPUT:Z" \
            -w "/OUTPUT" \
            -a stdout \
            -a stderr \
            "$IMAGE" /bin/sh -c "${INSTALL_COMMANDS}eval $RUN_COMMAND"
        echo "Output directory: $OUTPUT_DIR"
        ;;
    shell)
        podman run -it --rm \
            -v "$OUTPUT_DIR:/OUTPUT:Z" \
            -w "/OUTPUT" \
            -e "PS1=\[\033[1;32m\]1shot\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ " \
            "$IMAGE" /bin/sh -c "${INSTALL_COMMANDS}exec /bin/sh"
        echo "Output directory: $OUTPUT_DIR"
        ;;
    help|"")
        show_help
        ;;
    *)
        echo "Unknown command: $COMMAND_TYPE"
        show_help
        exit 1
        ;;
esac

