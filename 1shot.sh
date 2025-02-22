#!/bin/bash
set -e  # Exit on any error
trap 'echo "An error occurred. Exiting..."; exit 1' ERR

show_help() {
    echo "Usage: 1shot <command> [options]

Commands:
  run <command>        Run a single command in a container
  exec <file>          Execute a file
  shell                Start an interactive shell in a container
  help                 Show this help message

Options:
  -i, --image <image>  Specify a target image, default=alpine:latest
  -o, --output-dir <path>    Specify the output directory (default: current directory)
  -c, --cap-add [privs]       List of priviliges to add

Install packages:
  --from-apk   [pkgs]  Install packages from apk
  --from-git   [repos] Download repositories from git
  --from-cargo [pkgs]  Install packages from cargo
  --from-uv    [pkgs]  Install packages from uv

Examples:

  Hello world example:
	1shot run \"uvx pycowsay 'Hello from 1shot!'\" --from-uv pycowsay

  Run an interactive shell:
  	1shot shell

  Run a Python3 interactive shell:
    1shot run \"python3 -i\" --from-apk python3 --from-uv numpy pandas

  Download a YouTube video:
    1shot run 'yt-dlp dQw4w9WgXcQ' --from-uv yt-dlp

  Convert an image format:
	1shot run 'convert input.png output.jpg' --from-apk imagemagick

  Perform a quick network scan:
    1shot run \"mtr -n -r -c 10 google.com\" --from-apk mtr -c NET_RAW NET_ADMIN

  Pretty print JSON:
  	echo '{\"foo\":\"bar\", \"baz\":[1,2,3]}' | 1shot run 'jq .' --from-apk jq

  Analyse JSON structure:
	1shot run \"jq -r 'paths | join(\".\")' input.json | sort -u\" --from-apk jq

  Video encoding:
	1shot run \"ffmpeg -i input.mp4 output.webm\" --from-apk ffmpeg

  DNS lookup:
	1shot run \"dig +short example.com A\" --from-apk bind-tools

  Convert markdown to PDF:
	1shot run \"pandoc -s input.md -o output.pdf\" --from-apk pandoc

  Scan for open ports:
	1shot run \"nmap -sT example.com\" --from-apk nmap --cap-add=NET_RAW --cap-add=NET_ADMIN

  Generate a QR Code:
	1shot run \"qrencode -o output.png 'https://example.com'\" --from-apk qrencode


Podman must be installed.
"
}

IMAGE="alpine:latest"
INSTALL_COMMANDS=""
COMMAND=""
OUTPUT_DIR="$(pwd)"
CAP_ADD=""

parse_args() {
    if [[ "$1" == "run" ]]; then
        shift
        RUN_COMMAND="$1"
        shift
    fi

    while [[ "$#" -gt 0 ]]; do
        case $1 in
 			-c|--cap-add)
                shift
                while [ -n "$1" ] && [[ "$1" != -* ]]; do
                    CAP_ADD+="--cap-add=$1 "
                    shift
                done
                ;;            -i|--image)
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
                APK_PACKAGES=""
                while [ -n "$2" ] && [[ "$2" != -* ]]; do
                    APK_PACKAGES+="$2 "
                    shift
                done
                if [ -n "$APK_PACKAGES" ]; then
                    INSTALL_COMMANDS+="apk add --no-cache $APK_PACKAGES && "
                fi
                shift
                ;;
            --from-git)
                INSTALL_COMMANDS+="apk add --no-cache git && "
                GIT_REPOS=""
                while [ -n "$2" ] && [[ "$2" != -* ]]; do
                    GIT_REPOS+="$2 "
                    shift
                done
                if [ -n "$GIT_REPOS" ]; then
                    for repo in $GIT_REPOS; do
                        INSTALL_COMMANDS+="git clone $repo && "
                    done
                fi
                shift
                ;;
            --from-cargo)
                INSTALL_COMMANDS+="apk add --no-cache cargo && "
                CARGO_PACKAGES=""
                while [ -n "$2" ] && [[ "$2" != -* ]]; do
                    CARGO_PACKAGES+="$2 "
                    shift
                done
                if [ -n "$CARGO_PACKAGES" ]; then
                    INSTALL_COMMANDS+="cargo install $CARGO_PACKAGES && "
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
        podman run --sig-proxy=true -i --rm \
            -v "$OUTPUT_DIR:/OUTPUT:Z" \
            -w "/OUTPUT" \
            -a stdout \
            -a stderr \
            $CAP_ADD \
            "$IMAGE" /bin/sh -c "${INSTALL_COMMANDS}eval $RUN_COMMAND"
        echo "Output directory: $OUTPUT_DIR"
        ;;
    shell)
        podman run -it --rm \
            -v "$OUTPUT_DIR:/OUTPUT:Z" \
            -w "/OUTPUT" \
            $CAP_ADD \
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

