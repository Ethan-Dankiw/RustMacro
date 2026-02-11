#![deny(warnings)]

mod r#virtual;
mod input;
mod r#macro;
mod common;
mod engine;
mod config;

use crate::config::Config;
use crate::engine::engine::MacroEngine;
use crate::r#macro::registry::MacroRegistry;
use crate::r#macro::scripts::animation_cancel::AnimationCancelMacro;
use crate::r#macro::scripts::gamble_coins::GambleCoinsMacro;
use crate::r#macro::scripts::skull_caverns::SkullCavernsMacro;
use crate::r#macro::scripts::tree_break::BreakTreeMacro;
use anyhow::{Context, Result};
use common::comm_bus::CommunicationBus;
use common::events::ApplicationEvent;
use input::monitor::InputListenerThread;
use std::sync::Arc;

fn main() -> Result<()> {
    // Load the config from disk
    let config = Arc::new(Config::new()?);

    // Print the mouse and keyboard values
    println!("Detected Config Entries:");
    println!(" - Mouse Input: {}", config.mouse_input);
    println!(" - Keyboard Input: {}\n", config.keyboard_input);

    // Create a shared communication channel between all the input listeners to send their inputs to the main thread
    println!("Attempting to initialize Shared Communication Bus between threads...");
    let shared_bus = Arc::new(CommunicationBus::<ApplicationEvent>::new());
    println!("Shared Communication Bus Initialized!\n");

    // Register the shutdown hook to close the application gracefully
    println!("Attempting to register shutdown hook...");
    match register_shutdown_hook(&shared_bus) {
        Ok(_) => println!("Shutdown hook registered successfully!\n"),
        Err(e) => {
            eprintln!("Error registering shutdown hook: {}\n", e);
        }
    }

    // Create a macro registry and load macros into it
    println!("Attempting to create and load Macro Registry...");
    let macro_registry = create_and_load_macro_registry();
    println!("Macro Registry loaded successfully!\n");

    // Start the input listeners thread that receive inputs from the physical mouse and keyboard
    println!("Attempting to start input listener threads...");
    match start_input_listeners(&shared_bus, &config) {
        Ok(_) => println!("Input listener threads started successfully!\n"),
        Err(e) => {
            anyhow::bail!("Failed to start the input listener threads: {}", e);
        }
    }

    // Start the macro engine
    println!("Attempting to start Macro Engine...");
    let engine = MacroEngine::new()?;
    println!("Macro Engine started successfully!\n");

    loop {
        // Receive data from the shared communicated bus
        let event = match shared_bus.receive_data() {
            Ok(event) => event,
            Err(err) => {
                eprintln!(
                    "Failed to receive data from the shared communication bus: {}",
                    err
                );
                continue;
            }
        };

        // Process the input event into a macro to run
        let macro_logic = match event {
            // If the input event is to quit the application, do so
            ApplicationEvent::QuitApp => {
                println!("Quitting Application...");
                break;
            }
            // If the input event is for a key press, check if that key maps to a macro trigger
            ApplicationEvent::KeyPress(key) => match macro_registry.get_macro_by_trigger(key) {
                Some(found_macro) => found_macro,
                None => continue,
            },
        };

        // Use the macro engine to execute the macro
        if let Err(e) = engine.execute_macro(macro_logic) {
            eprintln!("Failed to execute macro: {}", e);
        }
    }

    // Return successful exit of the program
    Ok(())
}

fn start_input_listeners(
    shared_bus: &Arc<CommunicationBus<ApplicationEvent>>,
    config: &Config,
) -> Result<()> {
    // Define the listeners that are being created, with the name of the thread and config value
    let listeners = [
        ("KeyboardListener", &config.keyboard_input),
        ("MouseListener", &config.mouse_input),
    ];

    // Loop over all the listeners being created
    for (name, event_file) in listeners {
        // Construct a device path from the event file
        let device_path = format!("/dev/input/{}", event_file);

        // Create and run the device listener thread
        InputListenerThread::new(
            name,
            device_path.as_str(),
            shared_bus.clone(),
        ).with_context(|| format!("Failed to create {} for {}", name, device_path))?
            .run()?;
    }
    Ok(())
}

fn register_shutdown_hook(shared_bus: &Arc<CommunicationBus<ApplicationEvent>>) -> Result<()> {
    // Register a shutdown hook to quit the application
    let shutdown_bus = shared_bus.clone();

    // Load a handler for when the process receives a quit signal
    let handler = ctrlc::set_handler(move || {
        println!("Received Shutdown Signal...");

        // Send the QuitApp event to the main loop
        if let Err(e) = shutdown_bus.send_data(ApplicationEvent::QuitApp) {
            // If the channel is closed, the app is probably already shutdown
            eprintln!("Failed to send application quit signal: {}", e);
        }
    });

    // Map the handler results
    match handler {
        Ok(_) => Ok(()),
        Err(err) => {
            anyhow::bail!("Failed to register shutdown hook: {}", err);
        }
    }
}

fn create_and_load_macro_registry() -> MacroRegistry {
    // Create the macro register so that input events can be mapped to macro execution
    let mut macro_registry = MacroRegistry::new();

    // Register the macros
    macro_registry.register(Arc::new(AnimationCancelMacro));
    macro_registry.register(Arc::new(BreakTreeMacro));
    macro_registry.register(Arc::new(SkullCavernsMacro));
    macro_registry.register(Arc::new(GambleCoinsMacro));

    // Return the initialised macro registry
    macro_registry
}
