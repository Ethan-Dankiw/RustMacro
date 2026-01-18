use crate::common::utils::sleep;
use crate::r#macro::traits::{GenericMacro, KeyboardRef, MacroAction, MouseRef};
use anyhow::Result;
use input_linux::Key;

pub struct AnimationCancelMacro;

impl GenericMacro for AnimationCancelMacro {
    fn macro_name(&self) -> &'static str {
        "AnimationCancel"
    }

    fn trigger_key(&self) -> Key {
        Key::F5
    }

    fn action_type(&self) -> MacroAction {
        MacroAction::ONCE
    }

    fn execute(&self, keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()> {
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
}
