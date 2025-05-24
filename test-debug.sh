#!/bin/bash

# Kill existing instances
pkill -f "flipclip" 2>/dev/null

# Load .env
if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

echo "Starting FlipClip with debug output..."
echo "===================================="
echo ""
echo "TESTING: Press ⌥⌘J once and watch the output"
echo "You should see:"
echo "  1. '--- Transform triggered ---'"
echo "  2. '(Ignoring duplicate event)' for the second event"
echo ""
echo "===================================="

./target/release/flipclip
