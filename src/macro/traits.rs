use crate::r#virtual::keyboard::VirtualKeyboard;
use crate::r#virtual::mouse::VirtualMouse;
use anyhow::Result;
use input_linux::Key;
use std::sync::{Arc, Mutex};

pub type KeyboardRef = Arc<Mutex<VirtualKeyboard>>;
pub type MouseRef = Arc<Mutex<VirtualMouse>>;

pub enum MacroAction {
    ONCE,
    INFINITE,
}

pub trait GenericMacro: Send + Sync {
    fn macro_name(&self) -> &'static str;
    fn trigger_key(&self) -> Key;
    fn action_type(&self) -> MacroAction;
    fn setup(&self, _keyboard_ref: KeyboardRef, _mouse_ref: MouseRef) -> Result<()> {
        Ok(())
    }
    fn execute(&self, keyboard_ref: KeyboardRef, mouse_ref: MouseRef) -> Result<()>;
}