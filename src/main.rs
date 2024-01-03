#![allow(dead_code)]
#![allow(unused)]

mod bar;
use bar::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::window::close_on_esc;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin {
            debug: false,
            filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
            ..default()
        })
        .insert_resource(DirectionalLightShadowMap { size: 1024 })
        .add_systems(Startup, (default_scene, create_bar_grid))
        .add_systems(Update, (rotate_bars))
        .add_systems(Update, close_on_esc)
        .run();
}

fn default_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient: ResMut<AmbientLight>,
) {
    ambient.brightness = 0.3;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Name::new("grid"),
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 20.0, -10.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // camera
    let transform = Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
}
