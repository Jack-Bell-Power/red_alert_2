use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::plugins::graphics::mesh::{grid_to_world, world_to_grid};

#[derive(Resource, Default)]
pub struct Placement {
    pub active: bool,
    pub size: (u16, u16),
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

    let diamond = create_diamond_mesh(placement.size);

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

fn create_diamond_mesh(size: (u16, u16)) -> Mesh {
    let tile_width = 64.0;
    let tile_height = 32.0;

    let (width, height) = size;

    let mut positions = Vec::<[f32; 3]>::new();
    let mut indices = Vec::<u32>::new();

    for y in 0..height {
        for x in 0..width {
            let center = grid_to_world(x as f32, y as f32, tile_width, tile_height);

            let start = positions.len() as u32;

            positions.push([center.x, center.y + tile_height * 0.5, 0.0]); // top

            positions.push([center.x + tile_width * 0.5, center.y, 0.0]); // right

            positions.push([center.x, center.y - tile_height * 0.5, 0.0]); // bottom

            positions.push([center.x - tile_width * 0.5, center.y, 0.0]); // left

            indices.extend([start, start + 1, start + 2, start, start + 2, start + 3]);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    mesh.insert_indices(Indices::U32(indices));

    mesh
}
