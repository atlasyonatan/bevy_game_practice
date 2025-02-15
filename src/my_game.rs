mod debug;
mod player;

use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use debug::DebugPlugin;
use player::{Player, PlayerPlugin};
use std::f32::consts::PI;

pub struct MyGamePlugin;

const PIXELS_PER_METER: f32 = 100.0;

const SCENE_SELECTOR: u8 = 0;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ));
        
        app.add_plugins(RapierDebugRenderPlugin::default().disabled());
        app.add_plugins(PlayerPlugin);
        app.add_plugins(DebugPlugin);
        app.add_systems(
            Startup,
            (
                add_camera_system,
                add_stops_system,
                add_players_system,
                add_rigid_body_toys_system,
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
            // RigidBody::Fixed, //todo: does this make a difference
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
    for i in 1..=5 {
        let radius = 10.0 * i as f32;
        commands.spawn((
            RigidBody::Dynamic,
            Collider::ball(radius),
            Transform::default().with_translation(Vec3::new(
                -100.0 * i as f32,
                100.0,
                z_index::TOY,
            )),
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
        ));
    }
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
            offset: CharacterLength::Absolute(0.1),
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
