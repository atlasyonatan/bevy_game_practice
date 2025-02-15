use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (
                players_jump_system.before(player_velocity_system),
                jump_cooldown_system.before(players_jump_system),
                player_fall_system.before(player_velocity_system),
                players_movement_system.before(player_velocity_system),
                player_velocity_system,
            ),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
struct JumpCooldown(f32);

fn players_jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            Option<&KinematicCharacterControllerOutput>,
        ),
        (With<Player>, Without<JumpCooldown>),
    >,
) {
    let jump_speed = 400.0;
    for (entity, mut velocity, controller_output) in query.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowUp)
            && controller_output.is_some_and(|output| output.grounded)
        {
            commands.entity(entity).insert(JumpCooldown(0.2));
            velocity.linvel.y += jump_speed;
        }
    }
}

fn jump_cooldown_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut JumpCooldown)>,
    mut commands: Commands,
) {
    for (entity, mut cooldown) in query.iter_mut() {
        cooldown.0 -= time.delta().as_secs_f32();
        if cooldown.0 <= 0.0 {
            commands.entity(entity).remove::<JumpCooldown>();
        }
    }
}

fn players_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let speed = 100.0;

    for mut velocity in query.iter_mut() {
        match (
            keyboard.pressed(KeyCode::ArrowLeft),
            keyboard.pressed(KeyCode::ArrowRight),
        ) {
            (true, false) => velocity.linvel.x = -speed,
            (false, true) => velocity.linvel.x = speed,
            _ => velocity.linvel.x = 0.0,
        };
    }
}

fn player_fall_system(
    time: Res<Time>,
    rapier_config_query: Query<&RapierConfiguration>,
    mut query: Query<(&mut Velocity, Option<&KinematicCharacterControllerOutput>), With<Player>>,
) {
    let config = rapier_config_query.single();

    for (mut velocity, controller_output) in &mut query {
        if controller_output.is_none_or(|output| !output.grounded) {
            let time_passed = time.delta().as_secs_f32();
            velocity.linvel.y += time_passed * config.gravity.y;
        }
    }
}

fn player_velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &Velocity), With<Player>>,
) {
    for (mut controller, velocity) in query.iter_mut() {
        let delta = velocity.linvel * time.delta().as_secs_f32();
        controller.translation = Some(match controller.translation {
            Some(vec) => Vec2::new(vec.x + delta.x, vec.y + delta.y),
            None => delta,
        })
    }
}
