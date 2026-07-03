use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

fn create_grid_mesh(radius: i32, tile_width: f32, tile_height: f32) -> Mesh {
    let mut positions = Vec::<[f32; 3]>::new();
    let mut indices = Vec::<u32>::new();

    let tw = tile_width * 0.5;
    let th = tile_height * 0.5;

    let mut index = 0;

    for x in -radius..=radius {
        for y in -radius..=radius {
            let c = grid_to_world(x, y, tile_width, tile_height);

            let top: [f32; 3] = [c.x, c.y + th, 0.0];
            let right = [c.x + tw, c.y, 0.0];
            let bottom = [c.x, c.y - th, 0.0];
            let left = [c.x - tw, c.y, 0.0];

            positions.push(top);
            positions.push(right);
            positions.push(bottom);
            positions.push(left);

            indices.extend_from_slice(&[
                index,
                index + 1,
                index + 1,
                index + 2,
                index + 2,
                index + 3,
                index + 3,
                index,
            ]);

            index += 4;
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::LineList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = create_grid_mesh(60, 64.0, 32.0);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.3)))),
    ));
}

pub fn grid_to_world(i: i32, j: i32, tile_w: f32, tile_h: f32) -> Vec2 {
    Vec2::new(
        (i - j) as f32 * (tile_w * 0.5),
        (i + j) as f32 * (tile_h * 0.5),
    )
}

pub fn world_to_grid(world: Vec2, tile_w: f32, tile_h: f32) -> IVec2 {
    let i = ((world.x / (tile_w * 0.5) + world.y / (tile_h * 0.5)) * 0.5).round() as i32;
    let j = ((world.y / (tile_h * 0.5) - world.x / (tile_w * 0.5)) * 0.5).round() as i32;
    IVec2::new(i, j)
}

