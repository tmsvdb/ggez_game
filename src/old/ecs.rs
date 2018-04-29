
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
	entities: Vec<Box<event::EventHandler>>,/*
	update_list: Vec<Box<event::EventHandler>>,
	draw_list: Vec<Box<event::EventHandler>>,
	keyboard_list: Vec<Box<event::EventHandler>>,
	mouse_list: Vec<Box<event::EventHandler>>,
	controller_list: Vec<Box<event::EventHandler>>,*/
}

impl ECS {

    pub fn new(_ctx: &mut Context, assets:Assets) -> GameResult<ECS> {
        let s = ECS { 
            assets: assets,
            system_time: SystemTime::now(),
            delta_time: 0.0,
        	entities: Vec::new(), /*
        	update_list: Vec::new(),
        	draw_list: Vec::new(),
        	keyboard_list: Vec::new(),
        	mouse_list: Vec::new(),
        	controller_list: Vec::new(),*/
        };
        Ok(s)
    }

    pub fn get_assets (&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub fn register_entity<T: event::EventHandler + 'static>(&mut self, entity: T) {
    	self.entities.push(Box::new(entity));
    }
    /*
    pub fn register_for_update<S: event::EventHandler + 'static>(&mut self, system: S) {
    	self.update_list.push(Box::new(system));
    }

    pub fn register_for_draw<S: event::EventHandler + 'static>(&mut self, system: S) {
    	self.draw_list.push(Box::new(system));
    }

    pub fn register_for_keyboard<S: event::EventHandler + 'static>(&mut self, system: S) {
    	self.keyboard_list.push(Box::new(system));
    }

    pub fn register_for_mouse<S: event::EventHandler + 'static>(&mut self, system: S) {
    	self.mouse_list.push(Box::new(system));
    }

    pub fn register_for_controller<S: event::EventHandler + 'static>(&mut self, system: S) {
    	self.controller_list.push(Box::new(system));
    }

    pub fn register_entity (&mut self, entity: Entity) {
    	self.entities.push(entity);
    }*/

    pub fn get_delta (&self) -> f32 {
        self.delta_time
    }

}

impl event::EventHandler for ECS{

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        self.delta_time = self.system_time.elapsed().unwrap().subsec_nanos() as f32 / 1000000000.0;
        self.system_time = SystemTime::now();

        for entity in &mut self.entities{
        	entity.update(_ctx);
        }
        /*
    	for system in &mut self.update_list {
    		system.system_update();
    		for entity in &mut self.entities {
    			system.update(_ctx, entity, self.delta_time);
    		}
    	}*/
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    	graphics::clear(ctx);

    	for entity in &mut self.entities{
        	entity.update(ctx);
        }
    	/*
    	for system in &mut self.draw_list {
    		for entity in &mut self.entities {
    			system.draw(ctx, entity, &self.assets);
    		}
    	}
    	*/
    	graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    	/*
        for system in &mut self.mouse_list {
    		for entity in &mut self.entities {
    			system.mouse_button_down_event(_ctx, entity, MouseButtonventData{button: button, x: x, y: y});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.mouse_button_down_event(_ctx, button, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        /*
        for system in &mut self.mouse_list {
    		for entity in &mut self.entities {
    			system.mouse_button_up_event(_ctx, entity, MouseButtonventData{button: button, x: x, y: y});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.mouse_button_down_event(_ctx, button, x, y);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        /*
        for system in &mut self.mouse_list {
    		for entity in &mut self.entities {
    			system.mouse_motion_event(_ctx, entity, MouseMotionEventData{state: _state, x: x, y: y, xrel: xrel, yrel: yrel});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.mouse_motion_event(_ctx, _state, x, y, xrel, yrel);
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
        /*
        for system in &mut self.mouse_list {
    		for entity in &mut self.entities {
    			system.mouse_wheel_event(_ctx, entity, MouseWheelEventData{x: x, y: y});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.mouse_wheel_event(_ctx, x, y);
        }
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        /*
        for system in &mut self.keyboard_list {
    		for entity in &mut self.entities {
    			system.key_down_event(_ctx, entity, KeyboardEventData{keycode: keycode, keymod: keymod, repeat: repeat});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.key_down_event(_ctx, keycode, keymod, repeat);
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        /*
        for system in &mut self.keyboard_list {
    		for entity in &mut self.entities {
    			//system.key_up_event(_ctx, entity, KeyboardEventData{keycode: keycode, keymod: keymod, repeat: repeat});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.key_up_event(_ctx, keycode, keymod, repeat);
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
        /*
        for system in &mut self.controller_list {
    		for entity in &mut self.entities {
    			system.controller_button_down_event(_ctx, entity, ControllerButtonEventData{btn: btn, instance_id: instance_id});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.controller_button_down_event(_ctx, btn, instance_id);
        }
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        /*
        for system in &mut self.controller_list {
    		for entity in &mut self.entities {
    			system.controller_button_up_event(_ctx, entity, ControllerButtonEventData{btn: btn, instance_id: instance_id});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.controller_button_up_event(_ctx, btn, instance_id);
        }
    }

    fn controller_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: i16, instance_id: i32) {
        /*
        for system in &mut self.controller_list {
    		for entity in &mut self.entities {
    			system.controller_axis_event(_ctx, entity, ControllerAxisEventData{axis: axis, value: value, instance_id: instance_id});
    		}
    	}
    	*/
    	for entity in &mut self.entities{
        	entity.controller_axis_event(_ctx, axis, value, instance_id);
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