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

    pub fn get_register_count(&self) -> usize {
        self.registry.len()
    }

    pub fn register(&mut self, task: Macro) {
        self.registry.insert(task.trigger_key(), task);
    }

    pub fn get_macro_by_trigger(&self, key: Key) -> Option<Macro> {
        self.registry.get(&key).cloned()
    }
}
