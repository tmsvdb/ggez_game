
use ggez::graphics;
use ggez::graphics::{ DrawParam, Rect, Point2};

//use ecs;

/// COMPONENT
pub struct Components {
	pub debug: Option<DebugStats>,
	pub navigation: Option<Navigation>,
	pub selection: Option<Selection>,
	pub spritesheet: Option<Spritesheet>,
}

impl Default for Components {
	fn default() -> Components { 
    	Components { 
    		debug: None,
    		navigation: None, 
    		selection: None, 
    		spritesheet: None 
    	}
    }
}
	
/// Implementation of Components

pub struct Selection {
	pub sprite: Sprite,
	pub selected: bool,
}

pub struct Navigation {
	pub arrived: bool,
	pub location: Point2,
	pub goto: Point2,
}

/*
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
}*/

pub struct Sprite {
	pub visible: bool,
	pub image: usize,
	pub transform: graphics::Rect,
	pub frame: graphics::Rect,
}

pub struct Spritesheet {
	pub sprite: Sprite,
	pub playing_animation: usize,
	pub animations: Vec<SpriteAnimation>,
}

pub struct SpriteAnimation {
	pub tag: String,
	pub time: f32,
	pub fps:f32,
	pub frames: Vec<graphics::Rect>,
}

pub struct DebugStats {
	pub font: graphics::Font,
	pub fps_str: String,
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