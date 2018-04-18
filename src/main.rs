//! The simplest possible example that does something.

extern crate ggez;

use std::env;
use std::path;

use ggez::conf;
use ggez::event;
use ggez::ContextBuilder;

use ecs::ECS;
use systems::{MoveSystem, DrawSystem, SpriteSystem};

mod ecs;
mod components;
mod systems;
mod scene;

/// MAIN

/// test git
pub fn main() {

    let cb = ContextBuilder::new("ecs test", "thomas")
        .window_setup(conf::WindowSetup::default().title("ECS TEST"))
        .window_mode(conf::WindowMode::default().dimensions(939, 678));
        //.add_resource_path(std::path::PathBuf::from("/assets"));
    let ctx = &mut cb.build().unwrap();

    // create new ecs engine
    let ecs: &mut ECS = &mut ecs::ECS::new(ctx).unwrap();

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
    ecs.register_for_update(MoveSystem);
	ecs.register_for_draw(DrawSystem);
	ecs.register_for_update(SpriteSystem::new());

	scene::Scene::setup(ctx, ecs);

	// do gameloop and use ecs engine as ggez event handler
    if let Err(e) = event::run(ctx, ecs) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
