use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

fn krei_kradan_maŝon(radiuso: i32, kahela_larĝo: f32, kahela_alto: f32) -> Mesh {
    let mut pozicioj = Vec::<[f32; 3]>::new();
    let mut indicoj = Vec::<u32>::new();

    let kl = kahela_larĝo * 0.5;
    let ka = kahela_alto * 0.5;

    let mut indico = 0;

    for x in -radiuso..=radiuso {
        for y in -radiuso..=radiuso {
            let c = krado_al_mondo(x, y, kahela_larĝo, kahela_alto);

            let supro: [f32; 3] = [c.x, c.y + ka, 0.0];
            let dekstre = [c.x + kl, c.y, 0.0];
            let malsupro = [c.x, c.y - ka, 0.0];
            let maldekstre = [c.x - kl, c.y, 0.0];

            pozicioj.push(supro);
            pozicioj.push(dekstre);
            pozicioj.push(malsupro);
            pozicioj.push(maldekstre);

            indicoj.extend_from_slice(&[
                indico,
                indico + 1,
                indico + 1,
                indico + 2,
                indico + 2,
                indico + 3,
                indico + 3,
                indico,
            ]);

            indico += 4;
        }
    }

    let mut maŝo = Mesh::new(
        PrimitiveTopology::LineList,
        RenderAssetUsages::default(),
    );

    maŝo.insert_attribute(Mesh::ATTRIBUTE_POSITION, pozicioj);
    maŝo.insert_indices(Indices::U32(indicoj));

    maŝo
}

pub fn setup(
    mut komandoj: Commands,
    mut maŝoj: ResMut<Assets<Mesh>>,
    mut materialoj: ResMut<Assets<ColorMaterial>>,
) {
    let maŝo = krei_kradan_maŝon(60, 64.0, 32.0);

    komandoj.spawn((
        Mesh2d(maŝoj.add(maŝo)),
        MeshMaterial2d(materialoj.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.3)))),
    ));
}

pub fn krado_al_mondo(i: i32, j: i32, tile_w: f32, tile_h: f32) -> Vec2 {
    Vec2::new(
        (i - j) as f32 * (tile_w * 0.5),
        (i + j) as f32 * (tile_h * 0.5),
    )
}

pub fn mondo_al_krado(world: Vec2, tile_w: f32, tile_h: f32) -> IVec2 {
    let i = ((world.x / (tile_w * 0.5) + world.y / (tile_h * 0.5)) * 0.5).round() as i32;
    let j = ((world.y / (tile_h * 0.5) - world.x / (tile_w * 0.5)) * 0.5).round() as i32;
    IVec2::new(i, j)
}

