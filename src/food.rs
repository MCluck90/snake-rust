use ggez::*;
use rand;
use rand::distributions::{IndependentSample, Range};

use player::*;
use util::{rect};

pub struct Food {
    pub x: f32,
    pub y: f32
}

impl Food {
    pub fn draw(&mut self, ctx: &mut Context, margin: f32) -> GameResult<()> {
        use graphics::*;

        set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0)).unwrap();
        rectangle(
            ctx,
            DrawMode::Fill,
            rect(
                self.x + margin,
                self.y + margin,
                SIZE,
                SIZE))?;
        
        Ok(())
    }

    pub fn rand_pos(&mut self, max_x: u32, max_y: u32, size: f32) {
        let x_between = Range::new(0u32, max_x);
        let y_between = Range::new(0u32, max_y);
        let mut rng = rand::thread_rng();
        self.x = x_between.ind_sample(&mut rng) as f32 * size;
        self.y = y_between.ind_sample(&mut rng) as f32 * size;
    }
}