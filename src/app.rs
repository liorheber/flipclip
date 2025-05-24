use anyhow::Result;
use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use enigo::{Enigo, Key, Direction, Settings, Keyboard};
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptsConfig {
    pub default_mode: String,
    pub modes: HashMap<String, String>,
}

impl Default for PromptsConfig {
    fn default() -> Self {
        let mut modes = HashMap::new();
        modes.insert("tldr".to_string(), "Return a 2‑sentence summary.".to_string());
        modes.insert("bullets".to_string(), "Return max 5 action‑oriented bullets.".to_string());
        modes.insert("direct".to_string(), "Rewrite, removing filler, ≤120 chars.".to_string());
        modes.insert("hebrew".to_string(), "Translate to Hebrew, keep formatting.".to_string());
        modes.insert("spanish".to_string(), "Translate to Spanish, keep formatting.".to_string());
        
        PromptsConfig {
            default_mode: "tldr".to_string(),
            modes,
        }
    }
}

pub struct App {
    pub clipboard: Clipboard,
    pub config: PromptsConfig,
    pub current_mode: String,
    pub config_path: PathBuf,
}

impl App {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()?;
        let config_path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".flipclip")
            .join("prompts.json");
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Load or create config
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content)?
        } else {
            let default_config = PromptsConfig::default();
            let json = serde_json::to_string_pretty(&default_config)?;
            fs::write(&config_path, json)?;
            default_config
        };
        
        let current_mode = config.default_mode.clone();
        
        Ok(App {
            clipboard,
            config,
            current_mode,
            config_path,
        })
    }
    
    pub fn reload_config(&mut self) -> Result<()> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)?;
            self.config = serde_json::from_str(&content)?;
        }
        Ok(())
    }
    
    pub fn cycle_mode(&mut self) -> Result<()> {
        self.reload_config()?;
        
        let modes: Vec<String> = self.config.modes.keys().cloned().collect();
        let current_index = modes.iter().position(|m| m == &self.current_mode).unwrap_or(0);
        let next_index = (current_index + 1) % modes.len();
        self.current_mode = modes[next_index].clone();
        
        Ok(())
    }
    
    pub fn transform_clipboard(&mut self) -> Result<()> {
        // Get clipboard content
        let text = self.clipboard.get_text()?;
        if text.trim().is_empty() {
            return Ok(());
        }
        
        // Get current prompt
        let prompt = self.config.modes
            .get(&self.current_mode)
            .cloned()
            .unwrap_or_else(|| "Summarize this text.".to_string());
        
        // Call OpenRouter API
        match call_openrouter(&text, &prompt) {
            Ok(transformed) => {
                // Update clipboard
                self.clipboard.set_text(&transformed)?;
                
                // Auto-paste the result with better timing
                if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
                    // Press and hold Cmd
                    let _ = enigo.key(Key::Meta, Direction::Press);
                    thread::sleep(Duration::from_millis(50));
                    
                    // Press V
                    let _ = enigo.key(Key::Unicode('v'), Direction::Click);
                    thread::sleep(Duration::from_millis(50));
                    
                    // Release Cmd
                    let _ = enigo.key(Key::Meta, Direction::Release);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        
        Ok(())
    }
}

fn call_openrouter(text: &str, prompt: &str) -> Result<String> {
    let api_key = env::var("OPENROUTER_API_KEY")
        .map_err(|_| anyhow::anyhow!("OPENROUTER_API_KEY environment variable not set"))?;
    
    let client = reqwest::blocking::Client::new();
    
    let body = serde_json::json!({
        "model": "qwen/qwen3-32b",
        "messages": [
            {
                "role": "system",
                "content": prompt
            },
            {
                "role": "user",
                "content": text
            }
        ],
        "max_tokens": 4096,
        "temperature": 0.3
    });
    
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;
    
    if !response.status().is_success() {
        let error_text = response.text()?;
        return Err(anyhow::anyhow!("API request failed: {}", error_text));
    }
    
    let response_json: serde_json::Value = response.json()?;
    
    // Debug print the response to see what's coming back
    eprintln!("API Response: {}", serde_json::to_string_pretty(&response_json)?);
    
    let result = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid API response format"))?
        .trim()
        .to_string();
    
    // Check if result is empty
    if result.is_empty() {
        eprintln!("Warning: API returned empty content");
        return Err(anyhow::anyhow!("API returned empty content"));
    }
    
    Ok(result)
}
