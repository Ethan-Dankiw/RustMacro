use crate::common::utils::sleep;
use crate::r#macro::traits::{GenericMacro, KeyboardRef, MacroAction, MouseRef};
use anyhow::Result;
use input_linux::Key;
use crate::r#macro::scripts::utils::cancel_animation;

pub struct AnimationCancelMacro;

impl GenericMacro for AnimationCancelMacro {
    fn macro_name(&self) -> &'static str {
        "AnimationCancel"
    }

    fn trigger_key(&self) -> Key {
        Key::F13
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

        // Cancel the animation
        cancel_animation(&keyboard_ref)?;

        // Indicate successful macro execution
        Ok(())
    }
}
