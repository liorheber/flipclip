# FlipClip

A macOS clipboard transformation tool that uses AI to transform your clipboard content with different modes.

## Features

- **Multiple transformation modes**: tldr, bullets, polite, direct, hebrew
- **Global hotkeys**: 
  - `⌥⌘J` - Transform clipboard with current mode
  - `⌥⌘M` - Cycle through modes
- **Visual feedback**: Floating notifications show current mode and status
- **Auto-paste**: Automatically pastes transformed content
- **Menu bar icon**: Simple 📋 icon for easy access

## Installation

1. Clone the repository
2. Make sure you have Rust installed
3. Set your OpenRouter API key:
   ```bash
   export OPENROUTER_API_KEY="your-api-key-here"
   ```
4. Build and run:
   ```bash
   ./run.sh
   ```

## Usage

1. Copy any text to your clipboard
2. Press `⌥⌘J` to transform it using the current mode
3. Press `⌥⌘M` to cycle between modes:
   - **tldr**: Create a 2-sentence summary
   - **bullets**: Create action-oriented bullet points
   - **polite**: Rewrite in a friendly professional tone
   - **direct**: Remove filler words, keep it under 120 chars
   - **hebrew**: Translate to Hebrew

## UI Feedback

FlipClip provides visual feedback through:
- **Terminal output**: Detailed status messages with emojis
- **Floating notifications**: Brief on-screen popups showing:
  - Current mode when cycling
  - Processing status during transformation
  - Success/error messages

## Configuration

The app stores its configuration in `~/.flipclip/prompts.json`. You can edit this file to customize the transformation prompts.

## Troubleshooting

If hotkeys trigger multiple times, the app now includes built-in debouncing that processes only the first event of each key press.

## Requirements

- macOS
- Rust 1.70+
- OpenRouter API key

## License

MIT License - see LICENSE file for details
