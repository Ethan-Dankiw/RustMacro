use crate::r#macro::traits::{GenericMacro, KeyboardRef, MacroAction, MouseRef};
use anyhow::Result;
use input_linux::Key;
use crate::common::utils::sleep;

pub struct GambleCoinsMacro;

impl GenericMacro for GambleCoinsMacro {
    fn macro_name(&self) -> &'static str {
        "GambleCoins"
    }

    fn trigger_key(&self) -> Key {
        Key::F24
    }

    fn action_type(&self) -> MacroAction {
        MacroAction::INFINITE
    }

    fn execute(&self, _keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()> {
        // Tap the mouse left click
        if let Ok(mouse) = mouse_ref.lock() {
            mouse.click_tap(Key::ButtonLeft)?;
        }

        // Sleep for a short while
        sleep(250);

        // Indicate successful macro execution
        Ok(())
    }
}
