mod app;

use anyhow::Result;
use app::App;
use cocoa::{
    appkit::{NSApp, NSApplication, NSApplicationActivationPolicyProhibited, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem},
    base::{nil},
    foundation::{NSAutoreleasePool, NSString},
};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use objc::{sel, sel_impl, msg_send};
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::Command;
use dotenv::dotenv;

fn show_ui_notification(title: &str, message: &str, duration: f32) {
    let _ = Command::new("./show-ui.swift")
        .arg(title)
        .arg(message)
        .arg(duration.to_string())
        .spawn();
}

fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Check if API key is set
    if std::env::var("OPENROUTER_API_KEY").is_err() {
        eprintln!("❌ Error: OPENROUTER_API_KEY environment variable not set");
        eprintln!("Please make sure you have a .env file with your API key or set it in your environment");
        std::process::exit(1);
    }
    
    // Create autorelease pool for Cocoa memory management
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        
        // Initialize the application
        let app_instance = NSApp();
        app_instance.setActivationPolicy_(NSApplicationActivationPolicyProhibited);
        
        println!("🚀 FlipClip is running!");
        println!("📋 Press ⌥⌘J to transform clipboard");
        println!("🔄 Press ⌥⌘M to cycle transformation mode");
        println!("🔑 Make sure OPENROUTER_API_KEY is set in your environment");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Create status bar item
        let status_bar = NSStatusBar::systemStatusBar(nil);
        let status_item = status_bar.statusItemWithLength_(-1.0); // Use variable width
        let button = status_item.button();
        if button != nil {
            let title = NSString::alloc(nil).init_str("📋");
            let _: () = msg_send![button, setTitle:title];
        }
        
        // Create menu
        let menu = NSMenu::new(nil);
        
        // Add quit item
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                NSString::alloc(nil).init_str("Quit FlipClip"),
                sel!(terminate:),
                NSString::alloc(nil).init_str("q"),
            );
        menu.addItem_(quit_item);
        
        status_item.setMenu_(menu);
        
        // Create app instance
        let app = Arc::new(Mutex::new(App::new()?));
        
        // Show initial mode
        let initial_mode = app.lock().unwrap().current_mode.clone();
        println!("\n✅ Current mode: {}", initial_mode);
        
        // Setup global hotkeys
        let manager = GlobalHotKeyManager::new()?;
        
        let transform_hotkey = HotKey::new(Some(Modifiers::ALT | Modifiers::SUPER), Code::KeyJ);
        let cycle_hotkey = HotKey::new(Some(Modifiers::ALT | Modifiers::SUPER), Code::KeyM);
        
        manager.register(transform_hotkey)?;
        manager.register(cycle_hotkey)?;
        
        // Shared state for preventing double triggers
        let transform_count = Arc::new(Mutex::new(0u32));
        let cycle_count = Arc::new(Mutex::new(0u32));
        
        // Handle hotkeys in a separate thread
        thread::spawn(move || {
            loop {
                if let Ok(event) = GlobalHotKeyEvent::receiver().recv() {
                    match event.id {
                        id if id == transform_hotkey.id() => {
                            let mut count = transform_count.lock().unwrap();
                            *count += 1;
                            
                            // Only process odd counts (first event of a pair)
                            if *count % 2 == 1 {
                                drop(count); // Release lock before processing
                                
                                if let Ok(mut app) = app.lock() {
                                    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                    println!("🔄 Processing clipboard with mode: {}", app.current_mode);
                                    
                                    // Show visual notification
                                    show_ui_notification("Processing...", &format!("Mode: {}", app.current_mode), 1.0);
                                    
                                    match app.transform_clipboard() {
                                        Ok(_) => {
                                            println!("✅ Clipboard transformed successfully!");
                                            println!("📋 Content pasted automatically");
                                            show_ui_notification("✅ Success", &format!("Transformed with {}", app.current_mode), 2.0);
                                        }
                                        Err(e) => {
                                            println!("❌ Error: {}", e);
                                            show_ui_notification("❌ Error", &e.to_string(), 3.0);
                                        }
                                    }
                                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                }
                            }
                        }
                        id if id == cycle_hotkey.id() => {
                            let mut count = cycle_count.lock().unwrap();
                            *count += 1;
                            
                            // Only process odd counts (first event of a pair)
                            if *count % 2 == 1 {
                                drop(count); // Release lock before processing
                                
                                if let Ok(mut app) = app.lock() {
                                    if let Err(e) = app.cycle_mode() {
                                        eprintln!("❌ Error cycling mode: {}", e);
                                        show_ui_notification("❌ Error", &e.to_string(), 2.0);
                                    } else {
                                        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                        println!("🔄 Mode changed to: {}", app.current_mode);
                                        // Get the description for the current mode
                                        if let Some(description) = app.config.modes.get(&app.current_mode) {
                                            println!("📝 Description: {}", description);
                                            show_ui_notification(&format!("Mode: {}", app.current_mode), description, 2.5);
                                        }
                                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
        
        // Run the application
        app_instance.run();
    }
    
    Ok(())
}
