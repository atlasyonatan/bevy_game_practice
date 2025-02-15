use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const DEBUG_PHYSICS_SNAPSHOT_KEY: KeyCode = KeyCode::KeyD;
const TOGGLE_DEBUG_RENDERER_KEY: KeyCode = KeyCode::Backquote;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                debug_kinematic_character_controller_output_system,
                toggle_debug_renderer_system,
            ),
        );
    }
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

fn toggle_debug_renderer_system(
    mut renderer: ResMut<DebugRenderContext>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(TOGGLE_DEBUG_RENDERER_KEY) {
        renderer.enabled = !renderer.enabled;
    }
}
