use crate::common::comm_bus::CommunicationBus;
use crate::common::thread::NamedThread;
use crate::r#macro::traits::{KeyboardRef, MouseRef};
use crate::r#macro::Macro;
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
        // Clone the communication bus so the thread get its own pointer
        let bus = self.bus.clone();

        // Get the name of the thread
        let name = self.thread.get_name();

        // Spawn the thread that executes oneshot macros
        println!("Attempting to create {} thread", name);
        self.thread.spawn(move || {
            println!("{} Thread Created!", name);

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

                // Log the macro being run
                println!("{} Macro Started", current_task.macro_name());

                // Indefinitely loop the execution of the macro
                loop {
                    // Run the setup for the macro
                    if let Err(e) = current_task.setup(keyboard.clone(), mouse.clone()) {
                        eprintln!("Failed to setup macro: {}", e);
                    };

                    // Execute one iteration of the macro
                    if let Err(e) = current_task.execute(keyboard.clone(), mouse.clone()) {
                        eprintln!("Failed to execute macro: {}", e);
                    }

                    // Check for an update to the currently running macro (non-blocking)
                    match bus.try_receive_data() {
                        // If a macro task was received
                        Ok(Some(new_task)) => {
                            // Log the macro being stopped
                            println!("{} Macro Stopped", current_task.macro_name());

                            // If the macro is the same as the current macro
                            if new_task.trigger_key() == current_task.trigger_key() {
                                // Toggle the macro by exiting the execution loop and wait for new macro
                                break;
                            }

                            // Log the macro being run
                            println!("{} Macro Started", new_task.macro_name());

                            // Switch the current task for the new one
                            current_task = new_task;

                            // Continue with the execution of the new task
                            continue;
                        }

                        // If there are no new messages, continue execution
                        Ok(None) => continue,

                        // If there was an error receiving a message
                        Err(e) => {
                            eprintln!("Failed to receive infinite macro to execute: {}", e);
                            return;
                        }
                    }
                }
            }

            // Log that the thread has finished
            eprintln!("{} Thread Finished!", name);
        })?;

        // Return thread spawn success
        Ok(())
    }

    pub fn execute(&self, task: Macro) -> Result<()> {
        // Send the task to the oneshot macro thread for execution
        self.bus.send_data(task)
    }
}
