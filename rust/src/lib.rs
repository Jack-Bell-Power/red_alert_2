use godot::prelude::*;

mod scripts;

struct RedAlert;

#[gdextension]
unsafe impl ExtensionLibrary for RedAlert {}
