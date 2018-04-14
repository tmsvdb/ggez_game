
use ecs;

/// COMPONENT
pub struct CustomComponents	{
	pub position: Option <Position>,
	pub velocity: Option <Velocity>,
}

impl Default for CustomComponents {
    fn default() -> CustomComponents { 
    	CustomComponents { position: None, velocity: None }
    }
}

impl ecs::Components for CustomComponents {
	type ComponentsType = CustomComponents;
}
/// Implementation of Components

pub struct Position {
	pub x: f32, 
	pub y: f32,
}

pub struct Velocity {
	pub x: f32, 
	pub y: f32,
}
