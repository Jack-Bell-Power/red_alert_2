use godot::prelude::*;

mod decoder;
mod errors;

struct RedAlert;

#[gdextension]
unsafe impl ExtensionLibrary for RedAlert {}
