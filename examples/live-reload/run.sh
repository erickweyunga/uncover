#!/bin/bash

echo "Live Reload Example for Uncovr Framework"
echo "========================================="
echo ""
echo "This example demonstrates tower-livereload integration."
echo ""
echo "IMPORTANT: For live reload to work, you need cargo-watch!"
echo ""

if ! command -v cargo-watch &> /dev/null
then
    echo "cargo-watch is not installed."
    echo ""
    echo "Install it with:"
    echo "  cargo install cargo-watch"
    echo ""
    exit 1
fi

echo "cargo-watch is installed"
echo ""
echo "Starting server with auto-reload..."
echo ""
echo "Instructions:"
echo "  1. Open http://127.0.0.1:3000 in your browser"
echo "  2. Edit templates/index.html"
echo "  3. Save the file"
echo "  4. Watch the browser auto-reload"
echo ""
echo "Press Ctrl+C to stop"
echo ""

cargo watch -x run -w src -w templates
