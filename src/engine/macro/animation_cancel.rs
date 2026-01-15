use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use anyhow::Result;
use input_linux::Key;
use crate::device::keyboard::VirtualKeyboard;
use crate::device::mouse::VirtualMouse;

type KeyboardRef = Arc<Mutex<VirtualKeyboard>>;
type MouseRef = Arc<Mutex<VirtualMouse>>;

pub fn perform_animation_cancel(keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()> {
    // Press mouse left-click
    if let Ok(mouse) = mouse_ref.lock() {
        mouse.click_down(Key::ButtonLeft)?;
    }

    // Sleep for a short while
    sleep(75);

    // Release mouse left-click
    if let Ok(mouse) = mouse_ref.lock() {
        mouse.click_release(Key::ButtonLeft)?;
    }

    // Sleep for a short while
    sleep(75);

    // Press the DELETE + RIGHT SHIFT + R keys to animation cancel
    if let Ok(keyboard) = keyboard_ref.lock() {
        keyboard.key_down(Key::Delete)?;
        keyboard.key_down(Key::RightShift)?;
        keyboard.key_down(Key::R)?;
    }

    // Sleep for a short while
    sleep(25);

    // Release the pressed down DELETE + RIGHT SHIFT + R keys
    if let Ok(keyboard) = keyboard_ref.lock() {
        keyboard.key_release(Key::Delete)?;
        keyboard.key_release(Key::RightShift)?;
        keyboard.key_release(Key::R)?;
    }
    
    // Indicate successful macro execution
    Ok(())
}

fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}