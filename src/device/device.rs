use std::io;
use anyhow::Result;
use input_linux::{EventKind, InputEvent, InputId, InputProperty, Key, KeyState, RelativeAxis};
use input_linux::sys::input_event;
use crate::device::uinput::{open_uinput, InputHandler};

pub struct VirtualDevice {
    identifier: InputId,
    handler: InputHandler,
}

impl VirtualDevice {
    pub fn new() -> Result<Self> {
        // Open the uinput file to create a virtual device
        let device = open_uinput()?;

        // Create an ID for the device
        let id = InputId {
            bustype: 0x03,
            vendor: 0x1234,
            product: 0x5678,
            version: 1,
        };

        // Return the virtual device
        Ok(Self { identifier: id, handler: device })
    }

    pub fn create(&self, name: &str) -> Result<()> {
        // Create a byte array to store the device name + null terminator
        let mut bytes = Vec::with_capacity(name.len() + 1);

        // Store the device name + null terminator
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);

        // Create the device
        self.handler.create(
            &self.identifier,
            &bytes,
            0,
            &[],
        )?;

        // Return a successful creation
        Ok(())
    }

    pub fn send_key(&self, key: Key, state: KeyState) -> Result<()> {
        // Create the input event
        let event = InputEvent {
            kind: EventKind::Key,
            code: key as u16,
            value: state.into(),
            time: Default::default(),
        };

        // Write the event to the device
        self.send_event(event)?;
        Ok(())
    }

    pub fn send_relative(&self, axis: RelativeAxis, value: i32) -> Result<()> {
        let event = InputEvent {
            kind: EventKind::Relative,
            code: axis as u16,
            value,
            time: Default::default(),
        };

        // Write the event to the device
        self.send_event(event)?;
        Ok(())
    }

    pub fn flush_events(&self) -> Result<()> {
        let event = InputEvent {
            kind: EventKind::Synchronize,
            code: 0,
            value: 0,
            time: Default::default(),
        };

        // Write the event to the device
        self.send_event(event)?;
        Ok(())
    }

    fn send_event(&self, event: InputEvent) -> Result<()> {
        // Write the event to the device
        self.handler.write(&[input_event::from(event)])?;
        Ok(())
    }

    pub fn enable_event(&self, event: EventKind) -> io::Result<()> {
        // Enable an event handler bit for the virtual device
        self.handler.set_evbit(event)
    }

    pub fn enable_key_press(&self, key: Key) -> io::Result<()> {
        // Enable a key event for the virtual device
        self.handler.set_keybit(key)
    }

    pub fn enable_mouse_axis(&self, axis: RelativeAxis) -> io::Result<()> {
        // Enable a mouse move event for the virtual device
        self.handler.set_relbit(axis)
    }

    pub fn enable_property(&self, property: InputProperty) -> io::Result<()> {
        self.handler.set_propbit(property)
    }
}