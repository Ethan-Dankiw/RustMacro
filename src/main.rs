mod channel;
mod device;
mod engine;
mod events;
mod input;

use crate::engine::handler::MacroEngine;
use crate::events::ApplicationEvent;
use crate::input::listener::InputListener;
use anyhow::{Result};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Create an input listener for the physical keyboard
    let keyboard = InputListener::new("/dev/input/event5")?;

    // Create a macro engine to process inputs from the keyboard
    let engine = MacroEngine::new()?;

    // Log that the system has initialised
    println!("System Initialised!");

    // Loop until application quit
    loop {
        // Block until input event is received
        match keyboard.next_event() {
            // If a valid event is received
            Ok(Some(event)) => {
                // If the event is for a macro toggle
                match event {
                    ApplicationEvent::ToggleMacro => engine.toggle_running()?,
                    ApplicationEvent::QuitApp => break,
                    _ => {}
                }
            }
            // If a valid, but unknown key event is received
            Ok(None) => continue,
            // If there was an error processing the event
            Err(e) => {
                eprintln!("Error receiving keyboard event: {}", e);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    Ok(())
}
