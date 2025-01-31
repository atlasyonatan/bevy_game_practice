use bevy::{
    // color::palettes::css::{BLUE, RED},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub struct MyGamePlugin;

const PIXELS_PER_METER: f32 = 100.0;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(PhysicsDebug::default());
        // app.add_systems(Update, toggle_physics_debug_system);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ));
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(Startup, add_camera_system);
        app.add_systems(Startup, add_stops_system);
        app.add_systems(Startup, add_players_system);
        app.add_systems(Startup, add_rigid_body_toys_system);
        app.add_systems(
            Update,
            (players_jump_system, players_jump_rise_system).chain(),
        );
        app.add_systems(Update, (players_movement_system, player_fall_system));
        app.add_systems(Update, debug_kinematic_character_controller_output_system);
    }
}

fn add_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

mod z_index {
    pub const PLAYER: f32 = 1.0;
    pub const STOP: f32 = 0.0;
    pub const TOY: f32 = 0.0;
}

// fn setup_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
//     rapier_config.gravity = Vec2::new(0.0, -9.81);
//   }

#[derive(Component)]
struct Stop;

fn add_stops_system(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let stops = [
        (Vec2::new(0.0, 0.0), Vec2::new(1000.0, 40.0), 0.0),
        (Vec2::new(100.0, 100.0), Vec2::new(100.0, 20.0), PI / 12.0),
    ];

    for (position, dimensions, cw_rotation) in stops {
        commands.spawn((
            Stop,
            Collider::cuboid(dimensions.x / 2.0, dimensions.y / 2.0),
            Transform {
                rotation: Quat::from_rotation_z(cw_rotation),
                translation: Vec3::new(position.x, position.y, z_index::STOP),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            RigidBody::Fixed, //todo: does this make a difference

                              // Mesh2d(meshes.add(dimensions)),
                              // MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ));
    }
}

fn add_rigid_body_toys_system(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 10.0),
        Transform::default().with_translation(Vec3::new(-100.0, 100.0, z_index::TOY)),
    ));
}

#[derive(Component)]
struct Player;

// #[derive(Component)]
// struct Velocity2D(Vec2);

#[derive(Component, Default)]
struct Jump(f32);

fn add_players_system(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Collider::ball(15.0),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            snap_to_ground: None,
            ..Default::default()
        },
        Restitution::coefficient(1.0), //how much energy is kept after collision
        Transform {
            rotation: Quat::from_rotation_z(0.0),
            translation: Vec3::new(-50.0, 200.0, z_index::PLAYER),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        Velocity::zero(),
        // Mesh2d(meshes.add(Circle::new())),
        // MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
    ));
}

fn players_jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), (With<Player>, Without<Jump>)>,
) {
    for (entity, controller_output) in &query {
        if keyboard.pressed(KeyCode::ArrowUp) && controller_output.grounded {
            commands.entity(entity).insert(Jump::default());
        }
    }
}

fn players_jump_rise_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump), With<Player>>,
) {
    let y_velocity = 50.0;
    let max_height = 100.0;

    for (entity, mut controller, mut jump) in &mut query {
        let mut distance_jumped_delta = time.delta().as_secs_f32() * y_velocity;

        if distance_jumped_delta + jump.0 >= max_height {
            distance_jumped_delta = max_height - jump.0;
            commands.entity(entity).remove::<Jump>();
        }

        jump.0 += distance_jumped_delta;

        controller.translation = Some(match controller.translation {
            Some(vec) => Vec2::new(vec.x, vec.y + distance_jumped_delta),
            None => Vec2::new(0.0, distance_jumped_delta),
        })
    }
}

fn players_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
) {
    let player_velocity = 100.0;

    for (mut controller, controller_output) in &mut query {
        if controller_output.is_none_or(|output| output.grounded) {
            let mut horizontal_movement = 0.0;

            if keyboard.pressed(KeyCode::ArrowLeft) {
                horizontal_movement -= time.delta().as_secs_f32() * player_velocity;
            }

            if keyboard.pressed(KeyCode::ArrowRight) {
                horizontal_movement += time.delta().as_secs_f32() * player_velocity;
            }

            controller.translation = Some(match controller.translation {
                Some(vec) => vec.with_x(horizontal_movement),
                None => Vec2::new(horizontal_movement, 0.0),
            })
        }
    }
}

fn player_fall_system(
    time: Res<Time>,
    rapier_config_query: Query<&RapierConfiguration>,
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &mut Velocity,
            &KinematicCharacterControllerOutput,
        ),
        (With<Player>, Without<Jump>),
    >,
) {
    let config = rapier_config_query.single();

    for (mut controller, mut velocity, controller_output) in &mut query {
        if !controller_output.grounded {
            let time_passed = time.delta().as_secs_f32();
            velocity.linvel.y += time_passed * config.gravity.y;
            let fall_distance = velocity.linvel.y * time_passed;

            controller.translation = Some(match controller.translation {
                Some(vec) => Vec2::new(vec.x, vec.y + fall_distance),
                None => Vec2::new(0.0, fall_distance),
            })
        }
    }
}

// fn input_player_movement_system(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut query: Query<
//         (
//             &mut KinematicCharacterController,
//             &KinematicCharacterControllerOutput,
//         ),
//         With<Player>,
//     >,
// ) {
//     let horizontal_speed = 1.0;
//     let jump_speed = 200.0;
//     let fall_speed = 2.0;

//     for (mut controller, output) in &mut query {
//         if keyboard.pressed(KeyCode::ArrowDown) {
//             controller.translation = Some(Vec2::new(0.0, fall_speed));
//         }
//         if keyboard.just_pressed(KeyCode::ArrowUp) && output.grounded {
//             controller.translation = Some(Vec2::new(0.0, -jump_speed));
//         }
//         if keyboard.pressed(KeyCode::ArrowLeft) {
//             controller.translation = Some(Vec2::new(-horizontal_speed, 0.0));
//         }
//         if keyboard.pressed(KeyCode::ArrowRight) {
//             controller.translation = Some(Vec2::new(horizontal_speed, 0.0));
//         }

//         // velocity.linvel = velocity.linvel.normalize_or_zero() * speed;

//         // velocity.linvel.x += direction.x;
//         // velocity.linvel.y -= direction.y;
//     }
// }

// fn move_player_system(mut query: Query<(&mut Transform, &), With<Player>>) {
//     for (mut transform, velocity) in &mut query {
//         transform.translation.x += velocity.0.x;
//         transform.translation.y += velocity.0.y;
//     }
// }

// fn player_stop_collision_system(players: Query<EntityRef, &Mesh2d, With<Player>>,)

// #[derive(Resource, Default)]
// struct PhysicsDebug(bool);

fn debug_kinematic_character_controller_output_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<
        (
            Entity,
            Option<&KinematicCharacterControllerOutput>,
            Option<&Velocity>,
        ),
        With<KinematicCharacterController>,
    >,
) {
    if keyboard.just_pressed(DEBUG_PHYSICS_SNAPSHOT_KEY) {
        for data in &query {
            println!("debug_kinematic_character_controller_output: {:?}", data);
        }
    }
}

const DEBUG_PHYSICS_SNAPSHOT_KEY: KeyCode = KeyCode::KeyD;

// fn toggle_physics_debug_system(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut debug: ResMut<PhysicsDebug>,
// ) {
//     if keyboard.just_pressed(DEBUG_PHYSICS_TOGGLE_KEY) {
//         debug.0 = !debug.0;
//         println!(
//             "Physics debug is {}",
//             match debug.0 {
//                 true => "ON",
//                 false => "OFF",
//             }
//         )
//     }
// }
