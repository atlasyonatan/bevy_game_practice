pub mod greeting;
pub mod my_game;

use bevy::prelude::*;
use my_game::MyGamePlugin;
// use greeting::GreetingsPlugin;

//mutiny remake?

fn main() {
    println!("Hello, world 3!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyGamePlugin)
        // .add_plugins(GreetingsPlugin)
        .run();
}
