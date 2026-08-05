use std::path::Path;

use godot::{
    classes::{ISprite2D, ImageTexture, ProjectSettings, Sprite2D, Texture2D},
    prelude::*,
};

use crate::decoder::shp::shp_reader::decode_shp_to_image;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct ShpSprite {
    // All textures decoded from the SHP file.
    // Each element represents one animation frame.
    textures: Vec<Gd<ImageTexture>>,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for ShpSprite {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            textures: Vec::new(),
            base,
        }
    }

    fn ready(&mut self) {
        self.load_shp();
    }
}

impl ShpSprite {
    fn load_shp(&mut self) {
        godot_print!("load_shp");
        if let Ok(textures) = decode_shp_to_image(
            Path::new(
                ProjectSettings::singleton()
                    .globalize_path("res://assets/shp/nuweapmk.shp")
                    .to_string()
                    .as_str(),
            ),
            Path::new(
                ProjectSettings::singleton()
                    .globalize_path("res://assets/pal/uniturb.pal")
                    .to_string()
                    .as_str(),
            ),
            true,
        ) {
            self.textures = textures;
            godot_print!("{:?}", self.textures);
        }
    }
}
