use bevy::{prelude::*, render::wireframe::Wireframe};
use rand::{prelude::StdRng, Rng, SeedableRng};

pub struct Boid;

pub struct Velocity(Vec3);

pub fn move_boids(time: Res<Time>, mut query: Query<(&mut Transform, &Boid, &Velocity)>) {
    for (mut transform, _, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

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
            .insert(Boid)
            .insert(Velocity(Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )))
            .insert(Wireframe);
    }
}
