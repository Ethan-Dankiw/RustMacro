use crate::bus::CommunicationBus;
use crate::event::parser::parse_input_event;
use crate::event::types::ApplicationEvent;
use crate::input::listener::InputListener;
use crate::thread::named_thread::NamedThread;
use anyhow::{Context, Result};
use std::sync::Arc;

type Data = ApplicationEvent;

pub struct InputListenerThread {
    thread: NamedThread,
    listener: Arc<InputListener>,
    bus: Arc<CommunicationBus<Data>>,
}

impl InputListenerThread {
    pub fn new(
        name: &'static str,
        device_path: &'static str,
        bus: Arc<CommunicationBus<Data>>,
    ) -> Result<Self> {
        // Create a new input listener
        let listener = Arc::new(InputListener::new(device_path)?);

        // Create the thread that will listen for inputs
        let thread = NamedThread::new(name)?;

        // Return the thread
        Ok(Self {
            listener,
            thread,
            bus,
        })
    }

    pub fn run(&self) -> Result<()> {
        println!("{} Thread Created!", self.thread.get_name());

        // Clone the communication bus and listener so the thread get its own pointer
        let bus = self.bus.clone();
        let listener = self.listener.clone();

        // Spawn the thread that will listen for inputs
        self.thread
            .spawn(move || {
                // Indefinitely listen for inputs from the device listener
                loop {
                    // Get the next event from the device input listener
                    let input = match listener.next_event() {
                        Ok(input) => input,
                        Err(err) => {
                            eprintln!(
                                "Failed to get next input from the device input listener: {err}"
                            );
                            continue;
                        }
                    };

                    // Process the event to prevent unwanted events from causing IPC overhead
                    let event = match parse_input_event(input) {
                        Some(event) => event,
                        None => continue,
                    };

                    // Send the input event to the main thread for event processing and handling
                    match bus.send_data(event) {
                        Ok(_) => (),
                        Err(err) => {
                            eprintln!("Failed to send event: {err}");
                            continue;
                        }
                    }
                }
            })
            .with_context(|| format!("Failed to spawn {} thread", self.thread.get_name()))?;

        // Return thread spawn success
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.thread.stop()
    }
}
