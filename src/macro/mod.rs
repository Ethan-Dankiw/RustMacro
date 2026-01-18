use crate::r#macro::generic::GenericMacro;
use std::sync::Arc;

pub mod action;
pub mod animation_cancel;
pub mod engine;
pub mod generic;
pub mod registry;
pub mod skull_caverns;

pub type Macro = Arc<dyn GenericMacro>;
