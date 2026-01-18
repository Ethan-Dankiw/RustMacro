use std::sync::Arc;
use crate::r#macro::generic::GenericMacro;

pub mod action;
pub mod animation_cancel;
pub mod registry;
pub mod skull_caverns;
pub mod generic;
pub mod engine;

pub type Macro = Arc<dyn GenericMacro>;