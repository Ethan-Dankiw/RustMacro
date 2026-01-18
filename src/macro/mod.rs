use crate::r#macro::traits::GenericMacro;
use std::sync::Arc;

pub mod registry;
pub mod scripts;
pub mod traits;

pub type Macro = Arc<dyn GenericMacro>;
