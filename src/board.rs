use bevy::{
    ecs::{Commands, Entity, IntoSystem, Query, Res, ResMut},
    input::Input,
    math::Vec3,
    pbr::PbrBundle,
    prelude::{
        shape::Plane, AppBuilder, Assets, Children, Color, Handle, Mesh, MouseButton,
        StandardMaterial, Transform,
    },
};
use bevy_mod_picking::{Group, PickState, PickableMesh};

use crate::pieces::Piece;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}

struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn create(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Plane { size: 1.0 }));

    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1.0, 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0.001, 0.001, 0.001).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: i, y: j });
        }
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    // Get entity under the cursor, if there is one
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

fn select_square(
    commands: &mut Commands,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece, &Children)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the square under the cursor and set it as the selected
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // Get the actual square. This ensures it exists and is a square
        if let Ok(square) = squares_query.get(*square_entity) {
            // Mark it as selected
            selected_square.entity = Some(*square_entity);

            if let Some(selected_piece_entity) = selected_piece.entity {
                let pieces_entity_vec: Vec<(Entity, Piece, Vec<Entity>)> = pieces_query
                    .iter_mut()
                    .map(|(entity, piece, children)| {
                        (entity, *piece, children.iter().copied().collect())
                    })
                    .collect();
                let pieces_vec = pieces_query
                    .iter_mut()
                    .map(|(_, piece, _)| *piece)
                    .collect();
                // Move the selected piece to the selected square
                if let Ok((_piece_entity, mut piece, _)) =
                    pieces_query.get_mut(selected_piece_entity)
                {
                    if piece.is_move_valid((square.x, square.y), pieces_vec) {
                        // Check if a piece of the opposite color exists in this square and despawn it
                        for (other_entity, other_piece, other_children) in pieces_entity_vec {
                            if other_piece.x == square.x
                                && other_piece.y == square.y
                                && other_piece.color != piece.color
                            {
                                // Despawn piece
                                commands.despawn(other_entity);
                                // Despawn all of it's children
                                for child in other_children {
                                    commands.despawn(child);
                                }
                            }
                        }
                        piece.x = square.x;
                        piece.y = square.y;
                    }
                }
                selected_square.entity = None;
                selected_piece.entity = None;
            } else {
                // Select the piece in the currently selected square
                for (piece_entity, piece, _) in pieces_query.iter_mut() {
                    if piece.x == square.x && piece.y == square.y {
                        // piece_entity is now the entity in the same square
                        selected_piece.entity = Some(piece_entity);
                        break;
                    }
                }
            }
        }
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}
