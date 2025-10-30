#!/bin/bash
# Wrapper script to run hydrapool_cli commands
# Usage: ./cli.sh <command> [args...]
# Examples:
#   ./cli.sh info
#   ./cli.sh gen-auth myuser mypassword
#   ./cli.sh --help

cd "$(dirname "$0")"
docker compose run --rm hydrapool-cli "$@"
