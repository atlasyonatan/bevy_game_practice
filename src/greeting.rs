use bevy::prelude::*;

pub struct GreetingsPlugin;

impl Plugin for GreetingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_persons_system);
        app.add_systems(Startup, greeting_system);
        app.add_systems(Update, greet_persons_system);
        app.insert_resource(GreetTimer(Timer::from_seconds(1.4, TimerMode::Repeating)));
    }
}

fn greeting_system() {
    println!("greetings!")
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_persons_system(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    //update timer with elapsed ticks
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    for name in &query {
        println!("Hello {}!", &name.0);
    }
}

fn add_persons_system(mut commands: Commands) {
    commands.spawn((Person, Name("🦀".to_string())));
    commands.spawn((Person, Name("🥕".to_string())));
    println!("added persons");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
