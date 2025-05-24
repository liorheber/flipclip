#!/bin/bash

echo "Starting FlipClip with clean state..."
echo "====================================="
echo ""

# Kill any existing FlipClip instances
pkill -f "flipclip" 2>/dev/null

# Load environment variables from .env file
if [ -f ".env" ]; then
    echo "Loading environment variables from .env..."
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "WARNING: No .env file found!"
    echo "Create a .env file with: OPENROUTER_API_KEY=your_api_key_here"
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

echo "API key loaded successfully"
echo ""

# Run FlipClip
cargo run --release
