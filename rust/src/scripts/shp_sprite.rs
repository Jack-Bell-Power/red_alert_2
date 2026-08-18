use godot::{
    classes::{ISprite2D, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct ShpSprite {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for ShpSprite {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
        }
    }
}
