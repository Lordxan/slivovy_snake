#!/usr/bin/env bash
# Simple HTTP server for serving the Snake game web build
# Usage: ./server.sh [port]

PORT=${1:-8080}

echo "Starting HTTP server on port $PORT..."
echo "Open http://localhost:$PORT in your browser"
echo ""

if command -v python3 >/dev/null 2>&1; then
    exec python3 -m http.server "$PORT" --directory web
elif command -v node >/dev/null 2>&1; then
    exec npx serve web -p "$PORT"
else
    echo "Error: No HTTP server found."
    echo "Install one of:"
    echo "  - python3 (pip install http.server)"
    echo "  - nodejs + npx serve"
    exit 1
fi
