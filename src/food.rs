use ggez::*;

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
}