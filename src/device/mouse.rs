use anyhow::Result;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use input_linux::{EventKind, InputProperty, Key, KeyState, RelativeAxis};
use crate::device::device::VirtualDevice;
use crate::device::position::Position;

pub struct VirtualMouse {
    device: VirtualDevice,
    current: RefCell<Position>,
}

impl VirtualMouse {
    pub fn new() -> Result<Self> {
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
        mouse.create("animation-cancel-virtual-mouse")?;

        // Return the virtual mouse device
        Ok(Self {
            device: mouse,
            current: RefCell::from(Position::new())
        })
    }

    pub fn reset_position(&self) {
        self.current.borrow_mut().move_to(0, 0);
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
        self.current.borrow_mut().move_y(-delta_px);
        self.device.send_relative(RelativeAxis::Y, -delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_down(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.current.borrow_mut().move_y(delta_px);
        self.device.send_relative(RelativeAxis::Y, delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_left(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.current.borrow_mut().move_x(-delta_px);
        self.device.send_relative(RelativeAxis::X, -delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }

    pub fn move_right(&self, delta_px: i32) -> Result<()> {
        // Move the mouse's relative X-axis some pixels to the left
        self.current.borrow_mut().move_x(delta_px);
        self.device.send_relative(RelativeAxis::X, delta_px)?;
        self.device.flush_events()?;
        Ok(())
    }
}
