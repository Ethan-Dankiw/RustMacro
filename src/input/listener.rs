use anyhow::{Context, Result};
use input_linux::{EvdevHandle, InputEvent};
use std::fs::File;

pub struct InputListener {
    handler: EvdevHandle<File>,
}

impl InputListener {
    pub fn new(device_path: &str) -> Result<Self> {
        // Open the file to the device path
        let file = File::open(device_path)
            .with_context(|| format!("Failed to open input device: {}", device_path))?;

        // Return the input listener with the event handler
        Ok(Self {
            handler: EvdevHandle::new(file),
        })
    }

    pub fn next_event(&self) -> Result<InputEvent> {
        // Get the next event from the event handler
        self.handler
            .read_event()
            .map(InputEvent::from)
            .map_err(|e| anyhow::anyhow!("Failed to read input: {}", e))
    }
}
