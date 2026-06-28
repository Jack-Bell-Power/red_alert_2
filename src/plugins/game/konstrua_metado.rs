use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::plugins::graphics::mesh::{krado_al_mondo, mondo_al_krado};

#[derive(Resource, Default)]
pub struct Metstato {
    pub aktiva: bool,
}

#[derive(Component)]
pub struct Metprezento;

pub fn metada_sistemo(
    mut komandoj: Commands,
    metstato: Res<Metstato>,
    antaŭvida_demando: Query<Entity, With<Metprezento>>,
    mut maŝoj: ResMut<Assets<Mesh>>,
    mut materialoj: ResMut<Assets<ColorMaterial>>,
) {
    if !metstato.aktiva || !antaŭvida_demando.is_empty() {
        return;
    }

    let rombego = krei_romban_maŝon(64.0, 32.0);

    komandoj.spawn((
        Mesh2d(maŝoj.add(rombego)),
        MeshMaterial2d(materialoj.add(ColorMaterial::from(Color::srgba(0.0, 1.0, 0.0, 0.45)))),
        Metprezento,
    ));
}

pub fn ĝisdatigi_antaŭvidan_lokigan_sistemon(
    fenestroj: Query<&Window>,
    kameraa_demando: Query<(&Camera, &GlobalTransform)>,
    mut antaŭvida_demando: Query<&mut Transform, With<Metprezento>>,
) {
    let Ok(fenestro) = fenestroj.single() else {
        return;
    };
    let Ok((kamerao, kameraa_transformo)) = kameraa_demando.single() else {
        return;
    };
    let Ok(mut antaŭvida_transformo) = antaŭvida_demando.single_mut() else {
        return;
    };

    let Some(kursora_pozicio) = fenestro.cursor_position() else {
        return;
    };
    let Ok(monda_pozicio) = kamerao.viewport_to_world_2d(kameraa_transformo, kursora_pozicio) else {
        return;
    };

    let kahela_larĝo = 64.0;
    let kahela_alto = 32.0;

    let krado = mondo_al_krado(monda_pozicio, kahela_larĝo, kahela_alto);
    let aligita = krado_al_mondo(krado.x, krado.y, kahela_larĝo, kahela_alto);

    antaŭvida_transformo.translation.x = aligita.x;
    antaŭvida_transformo.translation.y = aligita.y;
    antaŭvida_transformo.translation.z = 10.0;
}

fn krei_romban_maŝon(kahela_larĝo: f32, kahela_alto: f32) -> Mesh {
    let kl = kahela_larĝo * 0.5;
    let ka = kahela_alto * 0.5;

    let pozicioj = vec![
        [0.0, ka, 0.0],  // top
        [kl, 0.0, 0.0],  // right
        [0.0, -ka, 0.0], // bottom
        [-kl, 0.0, 0.0], // left
    ];

    let indicoj = vec![0, 1, 2, 0, 2, 3];

    let mut maŝo = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    maŝo.insert_attribute(Mesh::ATTRIBUTE_POSITION, pozicioj);
    maŝo.insert_indices(Indices::U32(indicoj));

    maŝo
}
