use crate::common::utils::sleep;
use crate::r#virtual::device::VirtualDevice;
use anyhow::Result;
use input_linux::{EventKind, Key, KeyState};

pub struct VirtualKeyboard {
    device: VirtualDevice,
}

impl VirtualKeyboard {
    pub fn new(name: &'static str) -> Result<Self> {
        // Create a virtual keyboard device
        let keyboard = VirtualDevice::new()?;

        // Enable key press events
        keyboard.enable_event(EventKind::Key)?;

        // Enable the following keys
        keyboard.enable_key_press(Key::Delete)?;
        keyboard.enable_key_press(Key::RightShift)?;
        keyboard.enable_key_press(Key::R)?;
        keyboard.enable_key_press(Key::W)?;
        keyboard.enable_key_press(Key::A)?;
        keyboard.enable_key_press(Key::S)?;
        keyboard.enable_key_press(Key::D)?;

        // Enable the macro toggle keybind
        keyboard.enable_key_press(Key::F5)?;
        keyboard.enable_key_press(Key::F6)?;

        // Create the virtual keyboard
        keyboard.create(name)?;

        // Return the virtual keyboard device
        Ok(Self { device: keyboard })
    }

    pub fn key_hold(&self, key: Key, duration_ms: u64) -> Result<()> {
        // Send the key down event
        self.key_down(key)?;

        // Wait to described delay in ms
        sleep(duration_ms);

        // Send the key up event
        self.key_release(key)?;
        Ok(())
    }

    pub fn key_down(&self, key: Key) -> Result<()> {
        // Send the key down event
        self.device.send_key(key, KeyState::PRESSED)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn key_release(&self, key: Key) -> Result<()> {
        // Send the key up event
        self.device.send_key(key, KeyState::RELEASED)?;
        self.device.flush_events()?;
        Ok(())
    }
}
