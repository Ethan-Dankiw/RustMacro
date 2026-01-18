use crate::common::utils::sleep;
use crate::r#virtual::device::VirtualDevice;
use anyhow::Result;
use input_linux::{EventKind, InputProperty, Key, KeyState, RelativeAxis};

pub struct VirtualMouse {
    device: VirtualDevice,
}

impl VirtualMouse {
    pub fn new(name: &'static str) -> Result<Self> {
        // Create a virtual mouse device
        let mouse = VirtualDevice::new()?;

        // Enable button press and mouse movement events
        mouse.enable_event(EventKind::Key)?;
        mouse.enable_event(EventKind::Relative)?;

        // Enable the pointer property for the mouse
        mouse.enable_property(InputProperty::Pointer)?;

        // Enable mouse button
        mouse.enable_key_press(Key::ButtonLeft)?;
        mouse.enable_key_press(Key::ButtonRight)?;
        mouse.enable_key_press(Key::ButtonMiddle)?;
        mouse.enable_key_press(Key::ButtonSide)?;
        mouse.enable_key_press(Key::ButtonExtra)?;

        // Enable mouse movements
        mouse.enable_mouse_axis(RelativeAxis::X)?;
        mouse.enable_mouse_axis(RelativeAxis::Y)?;
        mouse.enable_mouse_axis(RelativeAxis::Wheel)?;

        // Create the virtual mouse
        mouse.create(name)?;

        // Return the virtual mouse device
        Ok(Self { device: mouse })
    }

    pub fn click_tap(&self, key: Key) -> Result<()> {
        // Send the click down event
        self.click_down(key)?;

        // Wait to described delay in ms
        sleep(50);

        // Send the click up event
        self.click_release(key)?;
        Ok(())
    }

    pub fn click_down(&self, key: Key) -> Result<()> {
        // Send click pressed event
        self.device.send_key(key, KeyState::PRESSED)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn click_release(&self, key: Key) -> Result<()> {
        // Send click release event
        self.device.send_key(key, KeyState::RELEASED)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_up(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.device.send_relative(RelativeAxis::Y, -delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_down(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.device.send_relative(RelativeAxis::Y, delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_left(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.device.send_relative(RelativeAxis::X, -delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_right(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.device.send_relative(RelativeAxis::X, delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }
}
