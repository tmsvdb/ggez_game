
use ggez::graphics;
use ggez::graphics::{ DrawParam };

//use ecs;

/// COMPONENT
pub struct Components {
	pub position: Option<Position>,
	pub velocity: Option<Velocity>,
	pub graphics: Option<Graphics>,
}

impl Default for Components {
	fn default() -> Components { 
    	Components { position: None, velocity: None, graphics: None }
    }
}

/*
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
*/
/// Implementation of Components

pub struct Position {
	pub x: f32, 
	pub y: f32,
}

pub struct Velocity {
	pub x: f32, 
	pub y: f32,
}

pub struct Graphics {
	pub transform: DrawParam, //see *1
    pub draw: DrawEntity, 
}

pub enum DrawEntity {
	Image(graphics::Image),
	Text(graphics::Text),
	None,
}

impl Default for DrawEntity {
	fn default () -> DrawEntity { DrawEntity::None }
}

/*
	*1 Draw params description:
	========================
	pub struct DrawParam {
	    pub src: Rect - a portion of the drawable to clip, as a fraction of the whole image. Defaults to the whole image (1.0) if omitted.
	    pub dest: Point - the position to draw the graphic expressed as a Point.
	    pub rotation: f32 - orientation of the graphic in radians.
	    pub scale: Point - x/y scale factors expressed as a Point.
	    pub offset: Point  - specifies an offset from the center for transform operations like scale/rotation
	    pub shear: Point - x/y shear factors expressed as a Point.
	}
	This struct implements the Default trait
*/