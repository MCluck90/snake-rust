use ggez::*;
use ggez::event::*;
use std::time::Duration;

use util;

pub const SIZE: f32 = 25.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    x_speed: f32,
    y_speed: f32,
    time_since_update: u64,
    game_width: f32,
    game_height: f32
}

impl Player {
    pub fn new(game_width: f32, game_height: f32) -> Player {
        Player {
            x: (game_width / 2.0) - (game_width % SIZE),
            y: (game_height / 2.0) - (game_height % SIZE),
            x_speed: 0.0,
            y_speed: 0.0,
            time_since_update: 0,
            game_width: game_width,
            game_height: game_height
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let ms = util::to_ms(dt);
        self.time_since_update += ms;

        if self.time_since_update <= 150 {
            return;
        }
        self.time_since_update = 0;

        // Move
        self.x += self.x_speed;
        self.y += self.y_speed;

        // Wrap around the screen
        if self.x + SIZE > self.game_width {
            self.x = 0.0;
        } else if self.x < 0.0 {
            self.x = self.game_width - SIZE;
        }

        if self.y + SIZE > self.game_height {
            self.y = 0.0;
        } else if self.y < 0.0 {
            self.y = self.game_height - SIZE;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, margin: f32) -> GameResult<()> {
        use graphics::*;

        let player_color = Color::new(0.0, 1.0, 0.0, 1.0);
        let player_border_color = Color::new(0.2, 1.0, 0.2, 1.0);
        set_color(ctx, player_color).unwrap();
        rectangle(
            ctx,
            DrawMode::Fill,
            util::rect(
                self.x + margin,
                self.y + margin,
                SIZE,
                SIZE))?;
        set_color(ctx, player_border_color).unwrap();
        rectangle(
            ctx,
            DrawMode::Line,
            util::rect(
                self.x + margin,
                self.y + margin,
                SIZE,
                SIZE))?;
        Ok(())
    }

    pub fn on_input(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Left |
            Keycode::Right => {
                if self.x_speed == 0.0 {
                    self.y_speed = 0.0;
                    self.x_speed = SIZE;
                    if keycode == Keycode::Left {
                        self.x_speed *= -1.0;
                    }
                }
            },
            Keycode::Up |
            Keycode::Down => {
                if self.y_speed == 0.0 {
                    self.x_speed = 0.0;
                    self.y_speed = SIZE;
                    if keycode == Keycode::Up {
                        self.y_speed *= -1.0;
                    }
                }
            },
            _ => {}
        }
    }
}