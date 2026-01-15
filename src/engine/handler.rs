use crate::device::keyboard::VirtualKeyboard;
use crate::device::mouse::VirtualMouse;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::engine::r#macro::animation_cancel::perform_animation_cancel;

pub struct MacroEngine {
    is_running: Arc<AtomicBool>,
    keyboard: Arc<Mutex<VirtualKeyboard>>,
    mouse: Arc<Mutex<VirtualMouse>>,
}

impl MacroEngine {
    pub fn new() -> Result<Self> {
        // Define the member variables used for the MacroEngine
        let is_running = Arc::new(AtomicBool::new(false));
        let keyboard = Arc::new(Mutex::new(VirtualKeyboard::new()?));
        let mouse = Arc::new(Mutex::new(VirtualMouse::new()?));

        // Create the macro engine
        let engine = Self {
            is_running,
            keyboard,
            mouse,
        };

        // Spawn a persistent worker thread that handles the execution of macros
        engine.spawn_worker_thread();

        Ok(engine)
    }

    pub fn toggle_running(&self) -> Result<()> {
        // Get the current run state of the macro
        let running = self.is_running.load(Ordering::Relaxed);

        // If the macro is current running
        if running {
            println!("Stopping macro execution...");
            self.is_running.store(false, Ordering::Relaxed);
        } else {
            println!("Starting macro execution...");
            self.is_running.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn spawn_worker_thread(&self) {
        // Clone a reference the running state of the macro so the thread has a local copy
        let running = self.is_running.clone();

        // Clone a reference the virtual keyboard and mouse
        let mouse_ref = self.mouse.clone();
        let keyboard_ref = self.keyboard.clone();

        // Spawn the worker thread
        thread::spawn(move || {
            println!("MacroEngine worker thread started!");

            // Loop forever
            loop {
                // If the thread has no macros to execute
                if !running.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                // Execute the macro once
                if let Err(e) = perform_animation_cancel(keyboard_ref.clone(), mouse_ref.clone()) {
                    eprintln!("Error during macro execution: {}", e);
                }
            }
        });
    }
}
