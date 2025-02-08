mod player;

use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    math::vec2,
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use player::{Player, PlayerPlugin};
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
        app.add_plugins(RapierDebugRenderPlugin::default().disabled());
        app.add_plugins(PlayerPlugin);
        app.add_systems(
            Startup,
            (
                add_camera_system,
                add_stops_system,
                add_players_system,
                add_rigid_body_toys_system,
            ),
        );
        app.add_systems(
            Update,
            (
                debug_kinematic_character_controller_output_system,
                toggle_debug_renderer_system,
            ),
        );
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

#[derive(Component)]
struct Stop;

fn add_stops_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            Mesh2d(meshes.add(Rectangle::new(dimensions.x, dimensions.y))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ));
    }
}

fn add_rigid_body_toys_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let dimensions = vec2(20.0, 20.0);
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(dimensions.x / 2.0, dimensions.y / 2.0),
        Transform::default().with_translation(Vec3::new(-100.0, 100.0, z_index::TOY)),
        Mesh2d(meshes.add(Rectangle::new(dimensions.x, dimensions.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
    ));
}

fn add_players_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;
    commands.spawn((
        Player,
        Collider::ball(radius),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            ..Default::default()
        },
        Restitution::coefficient(1.0), //how much energy is kept after collision
        Transform {
            rotation: Quat::from_rotation_z(0.0),
            translation: Vec3::new(-50.0, 200.0, z_index::PLAYER),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        Velocity::zero(),
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
    ));
}

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
const TOGGLE_DEBUG_RENDERER_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_renderer_system(
    mut renderer: ResMut<DebugRenderContext>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(TOGGLE_DEBUG_RENDERER_KEY) {
        renderer.enabled = !renderer.enabled;
    }
}
