use crate::bus::CommunicationBus;
use crate::r#macro::generic::{GenericMacro, KeyboardRef, MouseRef};
use crate::r#macro::Macro;
use crate::thread::named_thread::NamedThread;
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
        Ok(Self {
            thread,
            bus,
        })
    }

    pub fn run(&self, keyboard: KeyboardRef, mouse: MouseRef) -> Result<()> {
        println!("{} Thread Created!", self.thread.get_name());
        
        // Clone the communication bus so the thread get its own pointer
        let bus = self.bus.clone();
        
        // Spawn the thread that executes oneshot macros
        self.thread.spawn(move || {
            // Indefinitely receive macros to execute
            loop {
                // Receive a macro to execute
                let task = match bus.receive_data() {
                    Ok(task) => task,
                    Err(err) => {
                        eprintln!("Failed to receive oneshot macro to execute: {}", err);
                        continue;
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
