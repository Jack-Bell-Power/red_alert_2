use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Building {
    // Current health points of the building.
    // When this value is less than or equal to zero, the building is destroyed.
    health: i32,
    // Maximum health points of the building.
    // Used when repairing the building or calculating the health percentage.
    max_health: i32,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Building {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            health: 0,
            max_health: 0,
            base,
        }
    }
}
