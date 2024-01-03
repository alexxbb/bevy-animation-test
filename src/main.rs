#![allow(dead_code)]
#![allow(unused)]

mod _main;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::window::close_on_esc;

#[derive(Component)]
struct Bar;

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, rotate)
        .add_systems(Update, update_bar)
        .run();
}

fn update_bar(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Handle<Mesh>, With<Bar>>,
    time: Res<Time>,
) {
    let time = time.elapsed_seconds();
    for (mesh_n, handle) in query.iter().enumerate() {
        if let Some(mut mesh) = meshes.get_mut(handle) {
            let VertexAttributeValues::Float32x3(ref mut data) = mesh
                .attribute_mut(Mesh::ATTRIBUTE_POSITION)
                .expect("Mesh attribute not found")
            else {
                return;
            };
            for point in data.iter_mut() {
                point[1] += (mesh_n as f32 + time * 10.0).sin() * 0.25;
            }
        }
    }
}

fn rotate(mut query: Query<&mut Transform, With<Bar>>, time: Res<Time>) {
    let elapsed = time.elapsed_seconds();
    for (i, mut tr) in query.iter_mut().enumerate() {
        let rot = (elapsed * 4.0 + i as f32).sin() * 0.1;
        tr.rotate_local_y(rot);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient: ResMut<AmbientLight>,
) {
    // plane
    ambient.brightness = 0.3;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: true,
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

    for i in 0..4 {
        let b = PbrBundle {
            mesh: meshes.add(shape::Box::new(2.0, 4.0, 2.0).into()),
            material: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            transform: Transform::from_xyz(-8.0 + (i * 5) as f32, 4.0, 0.0),
            ..default()
        };
        commands.spawn((b, Bar));
    }
}
