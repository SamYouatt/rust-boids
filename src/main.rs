use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::{
        camera::PerspectiveProjection,
        wireframe::{Wireframe, WireframePlugin},
    },
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};

use rust_boids::pan_orbit_camera;

fn main() {
    App::build()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup.system())
        .add_system(pan_orbit_camera::pan_orbit_camera.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // origin circle
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 4,
        })),
        material: materials.add(Color::ANTIQUE_WHITE.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    // surrounding circles
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.2,
            subdivisions: 4,
        })),
        material: materials.add(Color::AQUAMARINE.into()),
        transform: Transform::from_xyz(5.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.2,
            subdivisions: 4,
        })),
        material: materials.add(Color::FUCHSIA.into()),
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.2,
            subdivisions: 4,
        })),
        material: materials.add(Color::YELLOW.into()),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });

    // cones

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(pan_orbit_camera::PanOrbitCamera::default());
}
