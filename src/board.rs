use bevy::{
    ecs::{Commands, ResMut},
    math::Vec3,
    pbr::PbrBundle,
    prelude::{shape::Plane, Assets, Color, Mesh, StandardMaterial, Transform},
};
use bevy_mod_picking::PickableMesh;

pub struct Square {
    pub x: u8,
    pub y: u8,
}

pub fn create(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Plane { size: 1.0 }));
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0.001, 0.001, 0.001).into());

    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default());
        }
    }
}
