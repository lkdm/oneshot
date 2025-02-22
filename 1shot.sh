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

Podman must be installed.
"
}

IMAGE="alpine:latest"
INSTALL_COMMANDS=""
COMMAND=""
OUTPUT_DIR="$(pwd)"
parse_args() {
    while [[ "$#" -gt 0 ]]; do
        case $1 in
            -i|--image)
                IMAGE="$2"
                shift 2
                ;;
			-i|--output-dir)
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
                fi
                shift
                ;;
            --from-git)
                if [[ ! $INSTALL_COMMANDS =~ "apk add --no-cache git" ]]; then
                    INSTALL_COMMANDS+="apk add --no-cache git && "
                fi
                if [ -n "$2" ] && [[ "$2" != --* ]]; then
                    INSTALL_COMMANDS+="git clone $2 && "
                    shift
                fi
                shift
                ;;
            --from-cargo)
                if [[ ! $INSTALL_COMMANDS =~ "apk add --no-cache cargo" ]]; then
                    INSTALL_COMMANDS+="apk add --no-cache cargo && "
                fi
                if [ -n "$2" ] && [[ "$2" != --* ]]; then
                    INSTALL_COMMANDS+="cargo install $2 && "
                    shift
                fi
                shift
                ;;
            --from-uv)
                if [[ ! $INSTALL_COMMANDS =~ "pip3 install uv" ]]; then
                    INSTALL_COMMANDS+="apk add --no-cache python3 py3-pip && pip3 install uv && "
                fi
                if [ -n "$2" ] && [[ "$2" != --* ]]; then
                    INSTALL_COMMANDS+="uv install $2 && "
                    shift
                fi
                shift
                ;;
            *)
                COMMAND="$@"
                break
                ;;
        esac
    done
}
parse_args "$@"

OUTPUT_DIR="$(realpath "$OUTPUT_DIR")"

init() {
    podman machine init 2>&1 | grep -v "VM already exists" || true
    podman machine start 2>&1 | grep -v "already running" || true
}

case "${COMMAND%% *}" in
    run)
    	RUN_COMMAND="${COMMAND#run }"
        podman run -i --rm \
            -v "$OUTPUT_DIR:/OUTPUT:Z" \
            -w "/OUTPUT" \
            "$IMAGE" /bin/sh -c "${INSTALL_COMMANDS}${RUN_COMMAND}"
        echo "Output directory: $OUTPUT_DIR"
        ;;
    shell)
        podman run -it --rm \
            -v "$OUTPUT_DIR:/OUTPUT" \
            -w "/OUTPUT" \
            -e "PS1=\[\033[1;32m\]1shot\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]^ " \
            "$IMAGE" /bin/sh -c "${INSTALL_COMMANDS}exec /bin/sh"
        echo "Output directory: $OUTPUT_DIR"
        ;;
    help|"")
        show_help
        ;;
    *)
        echo "Unknown command: ${COMMAND%% *}"
        show_help
        exit 1
        ;;
esac

