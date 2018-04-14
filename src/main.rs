//! The simplest possible example that does something.

extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::ContextBuilder;
use std::env;
use std::path;


mod ecs;
mod components;
mod systems;

/// MAIN

use ecs::{ ECS, Entity };

use components::CustomComponents;
use components::*;
use systems::MoveSystem;

/// test git
pub fn main() {

    let cb = ContextBuilder::new("ecs test", "thomas")
        .window_setup(conf::WindowSetup::default().title("ECS TEST"))
        .window_mode(conf::WindowMode::default().dimensions(939, 678));
        //.add_resource_path(std::path::PathBuf::from("/assets"));
    let ctx = &mut cb.build().unwrap();

    // create new ecs engine
    let ecs: &mut ECS<CustomComponents> = &mut ecs::ECS::new(ctx).unwrap();

    // direct ctx asset path to asset folder
	if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }
    else {
        println!("Warning: coudn't mount path 'assets' to ctx filesystem");
    }

    // create and register systems
    if let Ok(m) = MoveSystem::new (ctx) {
    	ecs.register_system(m);
    }

    // create startup entities
	ecs.register_entity(Entity::new("Dragon", CustomComponents { 
		position: Some(Position { x:-939.0, y:0.0 }), 
		velocity: Some(Velocity { x:3.0, y:0.0 })
	}));

	// do gameloop and use ecs engine as ggez event handler
    if let Err(e) = event::run(ctx, ecs) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
