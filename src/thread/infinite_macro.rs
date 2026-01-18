use crate::bus::CommunicationBus;
use crate::r#macro::generic::{GenericMacro, KeyboardRef, MouseRef};
use crate::r#macro::Macro;
use crate::thread::named_thread::NamedThread;
use anyhow::Result;
use std::sync::Arc;

pub struct InfiniteMacroThread {
    thread: NamedThread,
    bus: Arc<CommunicationBus<Macro>>,
}

impl InfiniteMacroThread {
    pub fn new() -> Result<Self> {
        // Create a new communication bus for the thread
        let bus = Arc::new(CommunicationBus::<Macro>::new());

        // Create a new named thread to execute infinite macros
        let thread = NamedThread::new("InfiniteMacro")?;

        // Return the created thread
        Ok(Self { thread, bus })
    }

    pub fn run(&self, keyboard: KeyboardRef, mouse: MouseRef) -> Result<()> {
        println!("{} Thread Created!", self.thread.get_name());

        // Clone the communication bus so the thread get its own pointer
        let bus = self.bus.clone();

        // Spawn the thread that executes oneshot macros
        self.thread.spawn(move || {
            // Indefinitely receive macros to execute
            loop {
                // Receive a macro to execute (blocking)
                let mut current_task = match bus.receive_data() {
                    Ok(task) => task,
                    Err(err) => {
                        eprintln!("Failed to receive infinite macro to execute: {}", err);
                        break;
                    }
                };

                // Run the setup for the macro
                if let Err(e) = current_task.setup(keyboard.clone(), mouse.clone()) {
                    eprintln!("Failed to setup macro: {}", e);
                };

                // Indefinitely loop the execution of the macro
                loop {
                    // Execute one iteration of the macro
                    if let Err(e) = current_task.execute(keyboard.clone(), mouse.clone()) {
                        eprintln!("Failed to execute macro: {}", e);
                    }

                    // Check for an update to the currently running macro (non-blocking)
                    match bus.try_receive_data() {
                        // If a macro task was received
                        Ok(Some(new_task)) => {
                            // If the macro is the same as the current macro
                            if new_task.trigger_key() == current_task.trigger_key() {
                                // Toggle the macro by exiting the execution loop and wait for new macro
                                break;
                            }

                            // If the macro is a different one
                            // Run the setup for the new macro as there is no need to exit the loop yet
                            if let Err(e) = current_task.setup(keyboard.clone(), mouse.clone()) {
                                eprintln!("Failed to setup macro: {}", e);
                            };

                            // Switch the current task for the new one
                            current_task = new_task;

                            // Continue with the execution of the new task
                            continue;
                        },

                        // If there are no new messages, continue execution
                        Ok(None) => continue,

                        // If there was an error receiving a message
                        Err(e) => {
                            eprintln!("Failed to receive infinite macro to execute: {}", e);
                            return;
                        },
                    }
                }
            }
        })?;

        // Return thread spawn success
        Ok(())
    }

    pub fn execute(&self, task: Macro) -> Result<()> {
        // Send the task to the oneshot macro thread for execution
        self.bus.send_data(task)
    }

    pub fn stop(&self) -> Result<()> {
        // Attempt to join the spawned thread back to its parent
        self.thread.stop()
    }
}
