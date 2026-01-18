use crate::engine::workers::infinite::InfiniteMacroThread;
use crate::engine::workers::oneshot::OneshotMacroThread;
use crate::r#macro::traits::{KeyboardRef, MacroAction, MouseRef};
use crate::r#macro::Macro;
use crate::r#virtual::keyboard::VirtualKeyboard;
use crate::r#virtual::mouse::VirtualMouse;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct MacroEngine {
    // Stored threads
    oneshot_thread: OneshotMacroThread,
    infinite_thread: InfiniteMacroThread,

    // Stored virtual keyboard and mouse
    _keyboard: KeyboardRef,
    _mouse: MouseRef,
}

impl MacroEngine {
    pub fn new() -> Result<Self> {
        // Create the virtual keyboard and mouse
        let keyboard = Arc::new(Mutex::new(VirtualKeyboard::new("virtual-macro-keyboard")?));
        let mouse = Arc::new(Mutex::new(VirtualMouse::new("virtual-macro-mouse")?));

        // Spawn the threads that will execute the oneshot and infinite macro's
        let oneshot_thread = OneshotMacroThread::new()?;
        let infinite_thread = InfiniteMacroThread::new()?;

        // Run the new threads
        oneshot_thread.run(keyboard.clone(), mouse.clone())?;
        infinite_thread.run(keyboard.clone(), mouse.clone())?;

        Ok(Self {
            oneshot_thread,
            infinite_thread,
            _keyboard: keyboard,
            _mouse: mouse,
        })
    }

    pub fn execute_macro(&self, task: Macro) -> Result<()> {
        // Execute the macro on the correct thread
        match task.action_type() {
            MacroAction::ONCE => self.oneshot_thread.execute(task)?,
            MacroAction::INFINITE => self.infinite_thread.execute(task)?,
        }

        Ok(())
    }
}
