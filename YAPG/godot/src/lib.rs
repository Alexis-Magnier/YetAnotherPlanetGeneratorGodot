pub mod yapg;
pub mod planet;
pub mod component;
pub mod components;

use godot::prelude::*;

struct YAPG;

#[gdextension]
unsafe impl ExtensionLibrary for YAPG {}