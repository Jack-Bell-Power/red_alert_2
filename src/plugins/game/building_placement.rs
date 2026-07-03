use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::plugins::graphics::mesh::{grid_to_world, world_to_grid};

#[derive(Resource, Default)]
pub struct Placement {
    pub active: bool,
}

#[derive(Component)]
pub struct PreviewPlacement;

pub fn placement_system(
    mut commands: Commands,
    placement: Res<Placement>,
    preview_query: Query<Entity, With<PreviewPlacement>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !placement.active || !preview_query.is_empty() {
        return;
    }

    let diamond = create_diamond_mesh(64.0, 32.0);

    commands.spawn((
        Mesh2d(meshes.add(diamond)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.0, 1.0, 0.0, 0.45)))),
        PreviewPlacement,
    ));
}

pub fn update_preview_placement_system(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut preview_query: Query<&mut Transform, With<PreviewPlacement>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(mut preview_transform) = preview_query.single_mut() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let tile_width = 64.0;
    let tile_height = 32.0;

    let grid = world_to_grid(world_position, tile_width, tile_height);
    let aligned = grid_to_world(grid.x, grid.y, tile_width, tile_height);

    preview_transform.translation.x = aligned.x;
    preview_transform.translation.y = aligned.y;
    preview_transform.translation.z = 10.0;
}

fn create_diamond_mesh(tile_width: f32, tile_height: f32) -> Mesh {
    let tw = tile_width * 0.5;
    let th = tile_height * 0.5;

    let positions = vec![
        [0.0, th, 0.0],  // top
        [tw, 0.0, 0.0],  // right
        [0.0, -th, 0.0], // bottom
        [-tw, 0.0, 0.0], // left
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
