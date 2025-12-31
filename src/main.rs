use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, greet_hello_world)
        .run();
}

fn greet_hello_world() {
    println!("Hello, World!");
}
