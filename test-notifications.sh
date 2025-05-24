#!/bin/bash

echo "Testing macOS notification permissions..."
echo ""

# Test if terminal-notifier is available
if command -v terminal-notifier &> /dev/null; then
    echo "✓ terminal-notifier is installed"
    terminal-notifier -title "FlipClip Test" -message "If you see this, notifications work!"
else
    echo "✗ terminal-notifier not found. Installing it might help with notifications:"
    echo "  brew install terminal-notifier"
fi

echo ""
echo "Checking notification settings..."
echo "1. Open System Preferences → Notifications & Focus"
echo "2. Look for 'Terminal' or 'FlipClip' in the list"
echo "3. Make sure notifications are allowed"
echo ""
echo "If notifications still don't work:"
echo "- Try running FlipClip from Terminal.app instead of other terminals"
echo "- Make sure Do Not Disturb is turned off"
