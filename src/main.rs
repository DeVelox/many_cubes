mod cubes;
use crate::cubes::*;

use bevy::prelude::*;
use bevy::window::PresentMode::AutoNoVsync;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, (cubes, position_cubes).chain()))
        .add_systems(Update, rotate_cubes)
        .run();
}

fn setup(mut commands: Commands) {
    let light = (
        PointLight {
            intensity: 200_000_000.0,
            range: 1_000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::splat(100.0)),
    );

    let camera = (
        Camera3d::default(),
        Transform::from_xyz(100.0, 90.0, 100.0).looking_at(Vec3::new(0.0, -10.0, 0.0), Vec3::Y),
    );

    commands.spawn(light);
    commands.spawn(camera);
}
