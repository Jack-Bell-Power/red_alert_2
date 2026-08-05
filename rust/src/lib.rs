use godot::prelude::*;

mod decoder;
mod errors;
mod scripts;

struct RedAlert;

#[gdextension]
unsafe impl ExtensionLibrary for RedAlert {}
