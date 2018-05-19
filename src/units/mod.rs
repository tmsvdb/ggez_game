
use strategies::Animations;
use Location;
use Size;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::Texture;
use sdl2::rect::Rect;
//use sdl2::rect::Point;


pub struct Unit {
	animations: Option<Animations>,
    location: Box<Location>,
    size: Box<Size>,
}

impl Unit {

    pub fn update (&mut self, delta_time: f32){
        self.sprite_animations.update(delta_time);
        self.location.update(delta_time);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>) {
        let p = self.location.to_screen_position();
        let s = self.size.to_screen_size();
        self.sprite_animations.draw_current_frame(canvas, textures, Rect::new(p.x, p.y, s.x as u32, s.y as u32));
    }
}

