use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

fn create_iso_grid_mesh(size: i16, spacing: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);

    let mut positions = Vec::<[f32; 3]>::new();
    let mut indices = Vec::<u32>::new();

    let extent = size as f32 * spacing;
    let mut index: u32 = 0;

    // "/" diagonal lines
    for i in -size..=size {
        let offset = i as f32 * spacing;

        positions.push([offset - extent, -extent, 0.0]);
        positions.push([offset + extent, extent, 0.0]);

        indices.push(index);
        indices.push(index + 1);

        index += 2;
    }

    // "\" diagonal lines
    for i in -size..=size {
        let offset = i as f32 * spacing;

        positions.push([offset - extent, extent, 0.0]);
        positions.push([offset + extent, -extent, 0.0]);

        indices.push(index);
        indices.push(index + 1);

        index += 2;
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = create_iso_grid_mesh(100, 62.0);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.3)))),
        Transform {
            scale: Vec3::new(1.0, 0.5, 1.0),
            ..default()
        },
    ));
}
