use bevy::{prelude::*, render::wireframe::Wireframe};
use rand::{prelude::StdRng, Rng, SeedableRng};

pub struct Boid;

pub fn move_boids() {}

pub fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape_handle = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.2,
        rings: 2,
        depth: 1.0,
        ..Default::default()
    }));
    let mut rng = StdRng::from_entropy();

    for _ in 0..10 {
        commands
            .spawn_bundle(PbrBundle {
                mesh: shape_handle.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    ),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(
                    rng.gen_range(-5.0..5.0),
                    rng.gen_range(-5.0..5.0),
                    rng.gen_range(-5.0..5.0),
                ),
                ..Default::default()
            })
            .insert(Wireframe);
    }
}
