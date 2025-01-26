use bevy::prelude::*;

fn main() {
    println!("Hello, world 3!");
    App::new()
        .add_systems(Startup, greeting_system)
        .add_systems(Startup, add_persons_system)
        .add_systems(Update, greet_persons_system)
        .run();
}

fn greeting_system() {
    println!("greetings!")
}

fn greet_persons_system(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", &name.0);
    }
}

fn add_persons_system(mut commands: Commands) {
    commands.spawn((Person, Name("🦀".to_string())));
    commands.spawn((Person, Name("🥕".to_string())));
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
