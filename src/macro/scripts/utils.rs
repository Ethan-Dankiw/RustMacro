use crate::common::utils::sleep;
use crate::r#macro::traits::{KeyboardRef, MouseRef};
use anyhow::Result;
use input_linux::Key;

pub fn cancel_animation(keyboard_ref: &KeyboardRef) -> Result<()> {
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
    Ok(())
}

pub fn _fast_break_tree(keyboard_ref: &KeyboardRef, mouse_ref: &MouseRef) -> Result<()> {
    // Loop 3 times (2 times for the tree, 1 time for the trunk)
    for _ in 0..3 {
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
        sleep(125);

        // Cancel the animation
        cancel_animation(keyboard_ref)?;
    }
    Ok(())
}