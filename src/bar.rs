use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

#[derive(Component)]
pub(crate) struct Bar;

#[derive(Bundle)]
pub struct BarBundle {
    mesh: PbrBundle,
    marker: Bar,
}

impl BarBundle {
    fn new(origin: Vec3, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        BarBundle {
            mesh: PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(origin),
                ..default()
            },
            marker: Bar,
        }
    }
    fn mesh() -> impl Into<Mesh> {
        shape::Box::new(0.1, 4.0, 0.1)
    }
}

pub fn create_bar_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::rgb(0.9, 0.1, 0.1).into());
    const SIZE: i32 = 40;
    for i in -SIZE..SIZE {
        for j in -SIZE..SIZE {
            let i = i as f32 * 0.25;
            let j = j as f32 * 0.25;
            let origin = Vec3::new(i, 0., j);
            let mesh = meshes.add(BarBundle::mesh().into());
            let bar = BarBundle::new(origin, mesh.clone(), material.clone());
            commands.spawn(bar);
        }
    }
}

pub fn rotate_bars(mut query: Query<&mut Transform, With<Bar>>, time: Res<Time>) {
    let elapsed = time.elapsed_seconds();
    for (i, mut tr) in query.iter_mut().enumerate() {
        let rot = (elapsed * 5.5 + i as f32).sin() * 0.1;
        tr.rotate_local_y(rot);
    }
}

pub fn deform_bar(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Handle<Mesh>, With<Bar>>,
    time: Res<Time>,
) {
    let start = std::time::Instant::now();
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
                point[1] += ((mesh_n as f32 + time) * 10.).sin() * 0.05;
            }
        }
    }
}
