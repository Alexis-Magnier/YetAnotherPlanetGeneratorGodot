pub mod yapg;

use godot::prelude::*;

struct YAPG;

#[gdextension]
unsafe impl ExtensionLibrary for YAPG {}