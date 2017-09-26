extern crate ggez;

use ggez::*;
use ggez::event::*;
use ggez::graphics::{DrawMode, Rect};
use ggez::timer;
use std::time::Duration;

const PLAYER_SIZE: f32 = 25.0;
const MAX_SPEED: f32 = PLAYER_SIZE;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const GAME_MARGIN: f32 = 16.0;
const GAME_WIDTH: f32 = SCREEN_WIDTH * 0.8 - GAME_MARGIN;
const GAME_HEIGHT: f32 = SCREEN_HEIGHT - (GAME_MARGIN * 2.0);

fn rect_from_xy(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect {
        x: x + (w / 2.0),
        y: y + (h / 2.0),
        w: w,
        h: h
    }
}

struct MainState {
    pos_x: f32,
    pos_y: f32,
    x_speed: f32,
    y_speed: f32,
    time_since_player_moved: u64
}

impl MainState {
    fn new() -> MainState {
        MainState {
            pos_x: (GAME_WIDTH / 2.0) - (GAME_WIDTH % PLAYER_SIZE),
            pos_y: (GAME_HEIGHT / 2.0) - (GAME_HEIGHT % PLAYER_SIZE),
            x_speed: 0.0,
            y_speed: 0.0,
            time_since_player_moved: 0
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let nanos = dt.subsec_nanos() as u64;
        let ms = (1000*1000*1000 * dt.as_secs() + nanos)/(1000 * 1000);
        self.time_since_player_moved += ms;

        // Limit how quickly the player can move
        // while keeping them on the grid
        if self.time_since_player_moved > 150 {
            self.time_since_player_moved = 0;
            self.pos_x += self.x_speed;
            self.pos_y += self.y_speed;

            // Wrap around the screen
            if self.pos_x + PLAYER_SIZE > GAME_WIDTH {
                self.pos_x = 0.0;
            } else if self.pos_x < 0.0 {
                self.pos_x = GAME_WIDTH - PLAYER_SIZE;
            }

            if self.pos_y + PLAYER_SIZE > GAME_HEIGHT {
                self.pos_y = 0.0;
            } else if self.pos_y < 0.0 {
                self.pos_y = GAME_HEIGHT - PLAYER_SIZE;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let border_color = graphics::Color::new(0.2, 0.2, 0.2, 1.0);
        let game_board_color = graphics::Color::new(0.0, 0.0, 0.0, 1.0);
        let grid_color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
        let player_color = graphics::Color::new(0.0, 1.0, 0.0, 1.0);
        let player_border_color = graphics::Color::new(0.2, 1.0, 0.2, 1.0);

        // Clear the screen and render the border
        graphics::set_background_color(ctx, border_color);
        graphics::clear(ctx);

        // Render the game space background
        graphics::set_color(ctx, game_board_color).unwrap();
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            rect_from_xy(GAME_MARGIN, GAME_MARGIN, GAME_WIDTH, GAME_HEIGHT))?;

        // Render the player inside of the game space
        graphics::set_color(ctx, player_color).unwrap();
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            rect_from_xy(
                self.pos_x + GAME_MARGIN,
                self.pos_y + GAME_MARGIN,
                PLAYER_SIZE,
                PLAYER_SIZE))?;
        graphics::set_color(ctx, player_border_color).unwrap();
        graphics::rectangle(
            ctx,
            DrawMode::Line,
            rect_from_xy(
                self.pos_x + GAME_MARGIN,
                self.pos_y + GAME_MARGIN,
                PLAYER_SIZE,
                PLAYER_SIZE))?;


        /*
        let black = graphics::Color::new(0.0, 0.0, 0.0, 1.0);
        let green = graphics::Color::new(0.0, 1.0, 0.0, 1.0);
        let light_green = graphics::Color::new(0.2, 1.0, 0.2, 1.0);

        graphics::set_background_color(ctx, black);
        graphics::clear(ctx);

        graphics::set_color(ctx, green).unwrap();
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect {
                x: self.pos_x,
                y: self.pos_y,
                w: PLAYER_SIZE,
                h: PLAYER_SIZE
            })?;

        graphics::set_color(ctx, light_green).unwrap();
        graphics::rectangle(
            ctx,
            DrawMode::Line,
            Rect {
                x: self.pos_x,
                y: self.pos_y,
                w: PLAYER_SIZE,
                h: PLAYER_SIZE
            })?;
        graphics::present(ctx);

        timer::sleep_until_next_frame(ctx, 8);
        */
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left |
            Keycode::Right => {
                if self.x_speed == 0.0 {
                    self.y_speed = 0.0;
                    self.x_speed = MAX_SPEED;
                    if keycode == Keycode::Left {
                        self.x_speed *= -1.0;
                    }
                }
            },
            Keycode::Up |
            Keycode::Down => {
                if self.y_speed == 0.0 {
                    self.x_speed = 0.0;
                    self.y_speed = MAX_SPEED;
                    if keycode == Keycode::Up {
                        self.y_speed *= -1.0;
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn main() {
    let c = conf::Conf {
        window_title: "Snake Movement".to_string(),
        window_icon: "".to_string(),
        window_height: SCREEN_HEIGHT as u32,
        window_width: SCREEN_WIDTH as u32,
        vsync: true,
        resizable: false
    };
    let ctx = &mut Context::load_from_conf("snake_movement", "ggez", c).unwrap();
    let state = &mut MainState::new();
    event::run(ctx, state).unwrap();
}