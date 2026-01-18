use crate::device::keyboard::VirtualKeyboard;
use crate::device::mouse::VirtualMouse;
use crate::r#macro::action::MacroAction;
use crate::r#macro::generic::{GenericMacro, KeyboardRef, MouseRef};
use crate::r#macro::Macro;
use crate::thread::infinite_macro::InfiniteMacroThread;
use crate::thread::oneshot_macro::OneshotMacroThread;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct MacroEngine {
    // Stored threads
    oneshot_thread: OneshotMacroThread,
    infinite_thread: InfiniteMacroThread,

    // Stored virtual keyboard and mouse
    keyboard: KeyboardRef,
    mouse: MouseRef,
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
            keyboard,
            mouse,
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

    pub fn stop(&self) -> Result<()> {
        // Count the number of threads that have not stopped when they should have
        let mut error_count: u8 = 0;

        // Attempt to stop the oneshot macro thread
        if let Err(e) = self.oneshot_thread.stop() {
            eprintln!("Failed to stop the oneshot macro thread: {}", e);
            error_count += 1;
        }

        // Attempt to stop the infinite macro thread
        if let Err(e) = self.infinite_thread.stop() {
            eprintln!("Failed to stop the infinite macro thread: {}", e);
            error_count += 1;
        }

        // If more than one thread failed to stop
        if error_count > 0 {
            anyhow::bail!("Macro thread stopped");
        }

        Ok(())
    }
}
