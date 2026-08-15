use std::path::Path;

use godot::{
    classes::{ISprite2D, ProjectSettings, Sprite2D, Texture2D},
    prelude::*,
};

use crate::decoder::shp::shp_reader::decode_shp_to_image;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct ShpSprite {
    // All textures decoded from the SHP file.
    // Each element represents one animation frame.
    textures: Vec<Gd<Texture2D>>,
    // Time elapsed since the current frame started playing.
    elapsed: f32,
    // Duration that each frame should be displayed before switching.
    frame_time: f32,
    // Index of the current animation frame.
    current_frame: usize,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for ShpSprite {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            textures: Vec::new(),
            elapsed: 0.0,
            frame_time: 0.1,
            current_frame: 0,
            base,
        }
    }

    fn ready(&mut self) {
        self.load_shp();
    }

    fn process(&mut self, delta: f64) {
        self.update_texture(delta);
    }
}

impl ShpSprite {
    fn load_shp(&mut self) {
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
        }
    }

    fn update_texture(&mut self, delta: f64) {
        if self.textures.is_empty() {
            return;
        }

        self.elapsed += delta as f32;

        if self.elapsed < self.frame_time {
            return;
        }

        self.elapsed = 0.0;

        self.current_frame += 1;

        if self.current_frame >= self.textures.len() {
            self.current_frame = 0;
        }

        let texture = self.textures.get(self.current_frame).cloned();

        self.base_mut().set_texture(texture.as_ref());
    }
}
