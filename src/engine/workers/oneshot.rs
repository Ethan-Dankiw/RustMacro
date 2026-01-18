use crate::common::comm_bus::CommunicationBus;
use crate::common::thread::NamedThread;
use crate::r#macro::traits::{KeyboardRef, MouseRef};
use crate::r#macro::Macro;
use anyhow::Result;
use std::sync::Arc;

pub struct OneshotMacroThread {
    thread: NamedThread,
    bus: Arc<CommunicationBus<Macro>>,
}

impl OneshotMacroThread {
    pub fn new() -> Result<Self> {
        // Create a new communication bus for the thread
        let bus = Arc::new(CommunicationBus::<Macro>::new());

        // Create a new named thread to execute oneshot macros
        let thread = NamedThread::new("OneshotMacro")?;

        // Return the created thread
        Ok(Self { thread, bus })
    }

    pub fn run(&self, keyboard: KeyboardRef, mouse: MouseRef) -> Result<()> {
        // Clone the communication bus so the thread get its own pointer
        let bus = self.bus.clone();

        // Get the name of the thread
        let name = self.thread.get_name();

        // Spawn the thread that executes oneshot macros
        self.thread.spawn(move || {
            println!("{} Thread Created!", name);

            // Indefinitely receive macros to execute
            loop {
                // Receive a macro to execute
                let task = match bus.receive_data() {
                    Ok(task) => task,
                    Err(err) => {
                        eprintln!("Failed to receive oneshot macro to execute: {}", err);
                        break;
                    }
                };

                // Run the setup for the macro
                if let Err(e) = task.setup(keyboard.clone(), mouse.clone()) {
                    eprintln!("Failed to setup macro: {}", e);
                };

                // Run the actual macro
                if let Err(e) = task.execute(keyboard.clone(), mouse.clone()) {
                    eprintln!("Failed to execute macro: {}", e);
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
