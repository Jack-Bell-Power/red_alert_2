use godot::{classes::TileMapLayer, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct PlacementManager {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for PlacementManager {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let Some(parent) = self.base().get_parent() else {
            godot_error!("PlacementManager has no parent node.");
            return;
        };

        let ground = parent.get_node_as::<TileMapLayer>("iso_ground");
        let preview = parent.get_node_as::<TileMapLayer>("placement_preview");

        godot_print!("ground: {:?}", ground);
        godot_print!("preview: {:?}", preview);
    }
}
