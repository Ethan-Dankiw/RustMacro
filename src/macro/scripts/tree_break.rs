use crate::r#macro::scripts::utils::_fast_break_tree;
use crate::r#macro::traits::{GenericMacro, KeyboardRef, MacroAction, MouseRef};
use anyhow::Result;
use input_linux::Key;

pub struct BreakTreeMacro;

impl GenericMacro for BreakTreeMacro {
    fn macro_name(&self) -> &'static str {
        "BreakTree"
    }

    fn trigger_key(&self) -> Key {
        Key::F14
    }

    fn action_type(&self) -> MacroAction {
        MacroAction::ONCE
    }

    fn execute(&self, keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()> {
        // Break the selected tree quickly using animation cancelling
        _fast_break_tree(&keyboard_ref, &mouse_ref)?;

        // Indicate successful macro execution
        Ok(())
    }
}
