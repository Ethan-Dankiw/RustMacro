use crate::r#macro::Macro;
use input_linux::Key;
use std::collections::HashMap;

pub struct MacroRegistry {
    registry: HashMap<Key, Macro>,
}

impl MacroRegistry {
    pub fn new() -> Self {
        // Create a registry hashmap to store all the known macros
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register(&mut self, task: Macro) {
        // Clone the task pointer so that it can be printed
        let logic = task.clone();

        // Register the macro
        self.registry.insert(task.trigger_key(), task);
        println!("[{:?}] {} macro registered", logic.trigger_key(), logic.macro_name())
    }

    pub fn get_macro_by_trigger(&self, key: Key) -> Option<Macro> {
        self.registry.get(&key).cloned()
    }
}
