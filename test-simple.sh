#!/bin/bash

# Simple test script without all the debug output

# Kill existing instances
pkill -f "flipclip" 2>/dev/null

# Load .env
if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Run FlipClip
echo "Starting FlipClip..."
echo "Press ⌥⌘J to test transform (should only happen ONCE per press)"
echo "Watch the console output below:"
echo "================================"
./target/release/flipclip
