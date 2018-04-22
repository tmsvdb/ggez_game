
use ggez::graphics;
//use ggez::graphics::{Drawable, DrawParam};
use ggez::event;
use ggez::event::{Keycode, Mod, MouseButton, MouseState, Button, Axis};
use ggez::{Context, GameResult};
use std::time::{Duration, SystemTime};
use components::Components;
use assets::Assets;


/// GEZZ implementaion and ECS implemenation
pub struct ECS
{
    assets: Assets,
    system_time: SystemTime,
    delta_time: f32,
	entities: Vec<Entity>,
	update_systems: Vec<Box<SystemUpdate>>,
	draw_systems: Vec<Box<SystemDraw>>,
	keyboard_systems: Vec<Box<SystemKeyboard>>,
	mouse_systems: Vec<Box<SystemMouse>>,
	controller_systems: Vec<Box<SystemController>>,
}

impl ECS {

    pub fn new(_ctx: &mut Context, assets:Assets) -> GameResult<ECS> {
        let s = ECS { 
            assets: assets,
            system_time: SystemTime::now(),
            delta_time: 0.0,
        	entities: Vec::new(), 
        	update_systems: Vec::new(),
        	draw_systems: Vec::new(),
        	keyboard_systems: Vec::new(),
        	mouse_systems: Vec::new(),
        	controller_systems: Vec::new(),
        };
        Ok(s)
    }

    pub fn get_assets (&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub fn register_for_update<S: SystemUpdate + 'static>(&mut self, system: S) {
    	self.update_systems.push(Box::new(system));
    }

    pub fn register_for_draw<S: SystemDraw + 'static>(&mut self, system: S) {
    	self.draw_systems.push(Box::new(system));
    }

    pub fn register_for_keyboard<S: SystemKeyboard + 'static>(&mut self, system: S) {
    	self.keyboard_systems.push(Box::new(system));
    }

    pub fn register_for_mouse<S: SystemMouse + 'static>(&mut self, system: S) {
    	self.mouse_systems.push(Box::new(system));
    }

    pub fn register_for_controller<S: SystemController + 'static>(&mut self, system: S) {
    	self.controller_systems.push(Box::new(system));
    }

    pub fn register_entity (&mut self, entity: Entity) {
    	self.entities.push(entity);
    }

    pub fn get_delta (&self) -> f32 {
        self.delta_time
    }

}

impl event::EventHandler for ECS{

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        self.delta_time = self.system_time.elapsed().unwrap().subsec_nanos() as f32 / 1000000000.0;
        self.system_time = SystemTime::now();

    	for system in &mut self.update_systems {
    		system.system_update();
    		for entity in &mut self.entities {
    			system.update(_ctx, entity, self.delta_time);
    		}
    	}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    	graphics::clear(ctx);
    	for system in &mut self.draw_systems {
    		for entity in &mut self.entities {
    			system.draw(ctx, entity, &self.assets);
    		}
    	}
    	graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        for system in &mut self.mouse_systems {
    		for entity in &mut self.entities {
    			system.mouse_button_down_event(_ctx, entity, MouseButtonventData{button: button, x: x, y: y});
    		}
    	}
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        for system in &mut self.mouse_systems {
    		for entity in &mut self.entities {
    			system.mouse_button_up_event(_ctx, entity, MouseButtonventData{button: button, x: x, y: y});
    		}
    	}
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        for system in &mut self.mouse_systems {
    		for entity in &mut self.entities {
    			system.mouse_motion_event(_ctx, entity, MouseMotionEventData{state: _state, x: x, y: y, xrel: xrel, yrel: yrel});
    		}
    	}
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
        for system in &mut self.mouse_systems {
    		for entity in &mut self.entities {
    			system.mouse_wheel_event(_ctx, entity, MouseWheelEventData{x: x, y: y});
    		}
    	}
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        for system in &mut self.keyboard_systems {
    		for entity in &mut self.entities {
    			system.key_down_event(_ctx, entity, KeyboardEventData{keycode: keycode, keymod: keymod, repeat: repeat});
    		}
    	}
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        for system in &mut self.keyboard_systems {
    		for entity in &mut self.entities {
    			system.key_up_event(_ctx, entity, KeyboardEventData{keycode: keycode, keymod: keymod, repeat: repeat});
    		}
    	}
    }

    fn text_editing_event(&mut self, _ctx: &mut Context, text: String, start: i32, length: i32) {
        println!(
            "Text editing: {}, start {}, length: {}",
            text, start, length
        );
    }

    fn text_input_event(&mut self, _ctx: &mut Context, text: String) {
        println!("Text input: {}", text);
    }

    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        for system in &mut self.controller_systems {
    		for entity in &mut self.entities {
    			system.controller_button_down_event(_ctx, entity, ControllerButtonEventData{btn: btn, instance_id: instance_id});
    		}
    	}
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        for system in &mut self.controller_systems {
    		for entity in &mut self.entities {
    			system.controller_button_up_event(_ctx, entity, ControllerButtonEventData{btn: btn, instance_id: instance_id});
    		}
    	}
    }

    fn controller_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: i16, instance_id: i32) {
        for system in &mut self.controller_systems {
    		for entity in &mut self.entities {
    			system.controller_axis_event(_ctx, entity, ControllerAxisEventData{axis: axis, value: value, instance_id: instance_id});
    		}
    	}
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }
}

/// ENTITY
#[derive(Default)]
pub struct Entity {
	pub properties: EntityProperties,
	pub components: Components,
}

pub struct EntityProperties {
	pub name: String,
    //pub transform: DrawParam,
    //pub drawable: DrawEntity, 
}

impl Default for EntityProperties {
	fn default () -> EntityProperties {
		EntityProperties{ name: String::from("default") }
	}
}

impl Entity {

	pub fn new(properties: EntityProperties, components: Components) -> Entity {
		Entity {properties:properties, components: components }
	}
}

/// SYSTEM
pub trait SystemUpdate
{	
	fn system_update(&mut self);
	/// update the entity using this system
	fn update (&self, ctx: &mut Context, entity: &mut Entity, _delta_time: f32);
}

pub trait SystemDraw {
	/// each system has the ability to render to the ggez context
	fn draw (&self, ctx: &mut Context, entity: &mut Entity, _assets: &Assets);

}

pub struct KeyboardEventData {
	pub keycode: Keycode, 
	pub keymod: Mod, 
	pub repeat: bool
}

pub trait SystemKeyboard
{	
	// Handle key events.  These just map keyboard events
    fn key_down_event(&mut self, ctx: &mut Context, entity: &mut Entity, data: KeyboardEventData);
    fn key_up_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: KeyboardEventData);
}

pub struct MouseButtonventData {
	pub button: MouseButton,
    pub x: i32,
    pub y: i32,
}

pub struct MouseMotionEventData {
	pub state: MouseState,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

pub struct MouseWheelEventData {
    pub x: i32,
    pub y: i32,
}

pub trait SystemMouse
{
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: MouseButtonventData);
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: MouseButtonventData);
    fn mouse_motion_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: MouseMotionEventData);
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: MouseWheelEventData);
}

pub struct ControllerButtonEventData {
	pub btn: Button, 
	pub instance_id: i32
}
pub struct ControllerAxisEventData {
	pub axis: Axis,
    pub value: i16,
    pub instance_id: i32,
}
pub trait SystemController {
	fn controller_button_down_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: ControllerButtonEventData);
    fn controller_button_up_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data: ControllerButtonEventData);
    fn controller_axis_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data:ControllerAxisEventData);
}