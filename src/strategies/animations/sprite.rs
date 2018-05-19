

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use strategies::Updater;
use strategies::Animations;

pub struct SpriteAnimation {
    fps: i32,
    play_time: f32,
    show_frame: usize,
    frame_positions: Vec<Rect>,
}

pub struct Sprite {
    texture_index: usize,
    animations: Vec<SpriteAnimation>,    
    playing_animation: usize,
}

impl Sprite {
    pub fn single_animation (texture_index: usize, fps: i32, frame_positions: Vec<Rect>) -> Sprite {

        let mut new_animations = Vec::new();
        new_animations.push (SpriteAnimation {
            fps: fps,
            play_time: 0.0,
            show_frame: 0,
            frame_positions: frame_positions
        });

        Sprite {
            texture_index: texture_index,
            animations: new_animations,
            playing_animation: 0,
        }
    }
}

impl Animations for Sprite {

    type Animation = SpriteAnimation;
    type RenderParams = Rect;

    fn play (&mut self, animation_index: usize) {
        self.playing_animation = animation_index;
        self.animations[animation_index].play_time = 0.0;
    }    

    fn add (&mut self, animation: Self::Animation) -> usize {
        self.animations.push(animation);
        self.animations.len() - 1
    }

    fn draw (&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>, params: Self::RenderParams) {
        let animation = &self.animations[self.playing_animation];
        let frame = animation.frame_positions[animation.show_frame];
        let texture = textures[self.texture_index];
        let dest:Rect = params;
        canvas.copy_ex(&texture, Some(frame), Some(dest), 0.0, None, false, false).unwrap();
    }
}

impl Updater for Sprite {
    fn update (&mut self, delta_time: f32) {
        let animation = &mut self.animations[self.playing_animation];
        animation.play_time += delta_time * animation.fps as f32;
        animation.show_frame = animation.play_time as usize % animation.frame_positions.len();
    }
}