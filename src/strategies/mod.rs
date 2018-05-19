
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

pub mod animations;
pub mod location;
pub mod size;

pub trait Updater {
    fn update (&mut self, delta_time: f32);
}

pub trait Animations : Updater {
	type Animation; 
	type RenderParams;

    fn play(&mut self, animation_index: usize);
    fn add (&mut self, animation: Self::Animation) -> usize;
    fn draw (&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>, params: Self::RenderParams);
}

pub trait Location : Updater {
    fn set_position (&mut self, x: f32, y: f32);
    fn to_screen_position(&self) -> Point;
}

pub trait Size : Updater {
    fn set_size (&mut self, width: f32, height: f32);
    fn to_screen_size(&self) -> Point;
}