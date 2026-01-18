use crate::device::keyboard::VirtualKeyboard;
use crate::device::mouse::VirtualMouse;
use crate::r#macro::action;
use anyhow::Result;
use input_linux::Key;
use std::sync::{Arc, Mutex};

pub type KeyboardRef = Arc<Mutex<VirtualKeyboard>>;
pub type MouseRef = Arc<Mutex<VirtualMouse>>;

pub trait GenericMacro: Send + Sync {
    fn macro_name(&self) -> &'static str;
    fn trigger_key(&self) -> Key;
    fn action_type(&self) -> action::MacroAction;
    fn setup(&self, _keyboard_ref: KeyboardRef, _mouse_ref: MouseRef) -> Result<()> {
        Ok(())
    }
    fn execute(&self, keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()>;
}
