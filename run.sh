#!/bin/bash

# Load environment variables from .env file if it exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Check if API key is set
if [ -z "$OPENROUTER_API_KEY" ]; then
    echo ""
    echo "ERROR: OPENROUTER_API_KEY is not set!"
    echo "Please create a .env file with your API key:"
    echo "  echo 'OPENROUTER_API_KEY=your_api_key_here' > .env"
    echo ""
    exit 1
fi

# Build and run in release mode
echo "Building FlipClip..."
cargo build --release

echo "Running FlipClip..."
echo "Press ⌥⌘J to transform clipboard"
echo "Press ⌥⌘M to cycle transformation mode"
echo "Look for the 📋 icon in your menu bar"
echo ""

# Run the app
./target/release/flipclip
