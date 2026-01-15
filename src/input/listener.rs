
use anyhow::{Context, Result};
use input_linux::{EvdevHandle, Event, EventKind, Key, KeyState};
use std::fs::File;
use crate::events::ApplicationEvent;


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

    pub fn next_event(&self) -> Result<Option<ApplicationEvent>> {
        // Get the next event from the event handler
        let event = match self.handler.read_event() {
            Ok(event) => event.into_event(),
            Err(err) => {
                eprintln!("Failed to read input event: {err}");
                return Ok(None);
            }
        };

        // Filter out events that are not key presses
        if event.kind != EventKind::Key || event.value != KeyState::PRESSED.into() {
            return Ok(None);
        }

        // If the F5 key was pressed
        if event.code == Key::F5 as u16 {
            return Ok(Some(ApplicationEvent::ToggleMacro))
        }

        // If no
        Ok(None)
    }
}