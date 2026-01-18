pub mod bus;
mod device;
mod engine;
mod event;
mod input;
mod r#macro;
mod thread;
mod utils;

use crate::bus::CommunicationBus;
use crate::event::types::ApplicationEvent;
use crate::r#macro::animation_cancel::AnimationCancelMacro;
use crate::r#macro::engine::MacroEngine;
use crate::r#macro::registry::MacroRegistry;
use crate::r#macro::skull_caverns::SkullCavernsMacro;
use crate::thread::input_listener::InputListenerThread;
use anyhow::Result;
use std::sync::Arc;

fn main() -> Result<()> {
    // Create a shared communication channel between all the input listeners to send their inputs to the main thread
    let bus = Arc::new(CommunicationBus::<ApplicationEvent>::new());
    println!("Shared Communication Bus Initialised!");

    // Create the macro register so that input events can be mapped to macro execution
    let mut registry = MacroRegistry::new();
    println!("Macro Registry Created!");

    // Register the macros
    registry.register(Arc::new(AnimationCancelMacro));
    registry.register(Arc::new(SkullCavernsMacro));
    println!(
        "{} Macro's Registered Successfully!",
        registry.get_register_count()
    );

    // Create an input listener thread for the physical mouse and keyboard
    let mouse_listener =
        InputListenerThread::new("MouseListener", "/dev/input/event3", bus.clone())?;
    let keyboard_listener =
        InputListenerThread::new("KeyboardListener", "/dev/input/event5", bus.clone())?;
    println!("Creating Listener Threads...");

    // Run the input listening threads
    mouse_listener.run()?;
    keyboard_listener.run()?;

    // Start the macro engine
    let engine = MacroEngine::new()?;

    loop {
        // Receive data from the shared communicated bus
        let event = match bus.receive_data() {
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
                println!("Quitting...");
                break;
            }
            // If the input event is for a key press, check if that key maps to a macro trigger
            ApplicationEvent::KeyPress(key) => match registry.get_macro_by_trigger(key) {
                Some(found_macro) => found_macro,
                None => continue,
            },
        };

        // Use the macro engine to execute the macro
        if let Err(e) = engine.execute_macro(macro_logic) {
            eprintln!("Failed to execute macro: {}", e);
        }
    }

    // // Create an input listener for the physical mouse
    // let mouse = InputListener::new("/dev/input/event3")?;
    //
    // // Log that the device listener has been created
    // println!("Device Listener Initialised!");
    //
    // // Loop until application quit
    // loop {
    //     // Block until input event is received
    //     match mouse.next_event() {
    //         // If a valid event is received
    //         Ok(Some(event)) => {
    //             // If the event is for a macro toggle
    //             match event {
    //                 ApplicationEvent::QuitApp => break,
    //                 ev => engine.handle_event(ev),
    //             }
    //         }
    //         // If a valid, but unknown key event is received
    //         Ok(None) => continue,
    //         // If there was an error processing the event
    //         Err(e) => {
    //             eprintln!("Error receiving mouse event: {}", e);
    //             sleep(100);
    //         }
    //     }
    // }

    // Join the listener threads so they are not orphaned
    mouse_listener.stop()?;
    keyboard_listener.stop()?;

    // Stop the macro engine
    engine.stop()?;

    // Return successful exit of the program
    Ok(())
}
