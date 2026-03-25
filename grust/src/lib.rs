use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
}

mod menu;
mod about;
mod secure;
mod project;