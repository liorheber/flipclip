# FlipClip — Native macOS "Clipboard → Magic" Menu‑Bar Utility

> **Purpose:** Copy any text, hit **⌥⌘ J** (Option + Command + J) and the clipboard is instantly replaced with a OpenRouter‑powered transformation (summary, bullets, polite‑rewrite, etc.). Hit **⌥⌘ M** to cycle modes.

---

## 🗂️ Contents

1. [Prerequisites](#prerequisites)
2. [Project Setup](#project-setup)
3. [Core Functionality](#core-functionality)
4. [Hot‑Key Registration](#hot-key-registration)
5. [System‑Tray UI](#system-tray-ui)
6. [User‑Defined Prompt Config](#user-defined-prompt-config)
7. [OpenRouter API Integration](#openrouter-api-integration)
8. [Packaging & Signing](#packaging--signing)
9. [Creating the .dmg Installer](#creating-the-dmg-installer)
10. [Smoke Test](#smoke-test)
11. [Next Steps](#next-steps)

Every numbered section is **self‑contained** and ends with a **verifiable success metric**.

---

## Prerequisites

- **macOS 12+** (Intel or Apple Silicon)
- **Rust toolchain** (`rustup`, stable channel)
- **Xcode Command‑Line Tools** (`xcode-select --install`)
- An **OpenRouter API key** (with access to the Qwen‑3b model or another text‑only model)

**Success metric ✅** → `rustc --version` prints a valid version (e.g. `rustc 1.78.0`).

---

## Project Setup

```bash
cargo new flipclip
cd flipclip
```

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
menubar = "0.8"
global_hotkey = "0.4"
arboard = "3.3"
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde_json = "1.0"
```

**Success metric ✅** → `cargo check` completes with **0 errors**.

---

## Core Functionality

Create `src/app.rs` containing the clipboard transform logic (placeholder below):

```rust
pub fn transform_clipboard() {
    // 1. read clipboard text
    // 2. build prompt
    // 3. call Qwen
    // 4. write result back
}
```

Export `transform_clipboard()` in `src/main.rs`.

**Success metric ✅** → `cargo test` (after adding a dummy test) passes.

---

## Hot‑Key Registration

In `main.rs`:

```rust
use global_hotkey::{hotkey, HotKeyManager};
let mgr = HotKeyManager::new();
mgr.register(hotkey!("⌥+⌘+J"))?; // transform
mgr.register(hotkey!("⌥+⌘+M"))?; // cycle mode
```

Grant Accessibility access on first run (macOS prompt).

**Success metric ✅** → Pressing **⌥⌘ J** logs a console message inside your app.

---

## System‑Tray UI\$1

---

## User‑Defined Prompt Config

FlipClip looks for a JSON file at `~/.flipclip/prompts.json` at startup. Example:

```json
{
  "default_mode": "tldr",
  "modes": {
    "tldr": "Return a 2‑sentence summary.",
    "bullets": "Return max 5 action‑oriented bullets.",
    "polite": "Rewrite in a friendly yet professional tone.",
    "direct": "Rewrite, removing filler, ≤120 chars.",
    "hebrew": "Translate to Hebrew, keep formatting."
  }
}
```

The app hot‑reloads this file whenever you press **⌥⌘ M**, so changes appear instantly.

**Success metric ✅** → Editing the JSON and pressing **⌥⌘ M** cycles through the new/edited modes.

---

## OpenRouter API Integration

Create a helper:

```rust
fn call_qwen(prompt: &str) -> anyhow::Result<String> {
    let body = serde_json::json!({
        "model": "qwen-3b",
        "prompt": prompt,
        "max_tokens": 60,
        "temperature": 0.3
    });
    let res: serde_json::Value = reqwest::blocking::Client::new()
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", std::env::var("OPENROUTER_API_KEY").unwrap()))
        .json(&body)
        .send()?
        .json()?;
    Ok(res["choices"][0]["text"].as_str().unwrap_or("").trim().to_owned())
}
```

**Success metric ✅** → Running `cargo run` and pressing **⌥⌘ J** replaces the clipboard with Qwen output.

---

## Packaging & Signing

```bash
cargo install cargo-bundle
cargo bundle --release
```

Code‑sign:

```bash
codesign -s "Developer ID Application: YOUR_NAME" -f --options runtime \
  --entitlements entitlements.plist \
  target/release/bundle/osx/FlipClip.app
```

**Success metric ✅** → `spctl --assess -vv FlipClip.app` returns `accepted`.

---

## Creating the .dmg Installer

```bash
brew install create-dmg
create-dmg --volname "FlipClip" \
  --window-pos 200 120 --window-size 480 220 \
  --icon-size 100 --icon "FlipClip.app" 120 100 \
  --app-drop-link 360 100 \
  FlipClip.dmg FlipClip.app
```

**Success metric ✅** → A file named `FlipClip.dmg` appears in the project root.

---

## Smoke Test

1. Double‑click `FlipClip.dmg` and drag **FlipClip.app** to **Applications**.
2. Launch the app; grant Accessibility permission.
3. Copy any paragraph, press **⌥⌘ J**.
4. Paste — you should see a summarised version.

**Success metric ✅** → Clipboard output changes in under 3 s.
