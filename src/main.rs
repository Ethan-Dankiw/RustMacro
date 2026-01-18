mod r#virtual;
mod input;
mod r#macro;
mod common;
mod engine;

use crate::engine::engine::MacroEngine;
use crate::r#macro::registry::MacroRegistry;
use crate::r#macro::scripts::animation_cancel::AnimationCancelMacro;
use crate::r#macro::scripts::skull_caverns::SkullCavernsMacro;
use anyhow::Result;
use common::comm_bus::CommunicationBus;
use common::events::ApplicationEvent;
use input::monitor::InputListenerThread;
use std::sync::Arc;

fn main() -> Result<()> {
    // Create a shared communication channel between all the input listeners to send their inputs to the main thread
    let shared_bus = Arc::new(CommunicationBus::<ApplicationEvent>::new());
    println!("Shared Communication Bus Initialised!");

    // Register the shutdown hook to close the application gracefully
    match register_shutdown_hook(&shared_bus) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error registering shutdown hook: {}", e);
        }
    }

    // Create a macro registry and load macros into it
    let macro_registry = create_and_load_macro_registry();

    // Start the input listeners thread that receive inputs from the physical mouse and keyboard
    match start_input_listeners(&shared_bus) {
        Ok(_) => {},
        Err(e) => {
            anyhow::bail!("Failed to start the input listener threads: {}", e);
        }
    }

    // Start the macro engine
    let engine = MacroEngine::new()?;

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

fn start_input_listeners(shared_bus: &Arc<CommunicationBus<ApplicationEvent>>) -> Result<()> {
    // Create the listener thread for the physical keyboard
    let keyboard_listener = InputListenerThread::new(
        "KeyboardListener",
        "/dev/input/event5",
        shared_bus.clone(),
    ).expect("Failed to create keyboard listener");

    // Start the listener to receive keyboard input events
    keyboard_listener.run()?;

    // Create the listener thread for the physical mouse
    let mouse_listener = InputListenerThread::new(
        "MouseListener",
        "/dev/input/event3",
        shared_bus.clone(),
    ).expect("Failed to create mouse listener");
    println!("Creating Listener Threads");

    // Start the listener to receive mouse input events
    mouse_listener.run()?;
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
    println!("Macro Registry Created!");

    // Register the macros
    macro_registry.register(Arc::new(AnimationCancelMacro));
    macro_registry.register(Arc::new(SkullCavernsMacro));
    println!("{} Macro's Registered Successfully!", macro_registry.get_register_count());

    // Return the initialised macro registry
    macro_registry
}
