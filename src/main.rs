extern crate ggez;

mod player;
mod util;

use ggez::*;
use ggez::event::*;
use std::time::Duration;
use player::Player;
use util::rect;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 590.0;
const GAME_MARGIN: f32 = 16.0;
const GAME_WIDTH: f32 = player::SIZE * 24.0;
const GAME_HEIGHT: f32 = player::SIZE * 22.0;

struct MainState {
    player: Player
}

impl MainState {
    fn new() -> MainState {
        MainState {
            player: Player::new(GAME_WIDTH, GAME_HEIGHT)
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        self.player.update(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use graphics::*;

        let border_color = Color::new(0.2, 0.2, 0.2, 1.0);
        let game_board_color = Color::new(0.0, 0.0, 0.0, 1.0);
        let grid_color = Color::new(1.0, 1.0, 1.0, 1.0);

        // Clear the screen and render the border
        set_background_color(ctx, border_color);
        clear(ctx);

        // Render the game space background
        set_color(ctx, game_board_color).unwrap();
        rectangle(
            ctx,
            DrawMode::Fill,
            rect(GAME_MARGIN, GAME_MARGIN, GAME_WIDTH, GAME_HEIGHT))?;

        // Render the grid
        set_color(ctx, grid_color).unwrap();
        let mut x = 0.0;
        while x < GAME_WIDTH {
            let mut y = 0.0;
            while y < GAME_HEIGHT {
                rectangle(
                    ctx,
                    DrawMode::Line,
                    rect(
                        x + GAME_MARGIN,
                        y + GAME_MARGIN,
                        player::SIZE,
                        player::SIZE
                    ))?;
                y += player::SIZE;
            }
            x += player::SIZE;
        }

        // Render the player inside of the game space
        self.player.draw(ctx, GAME_MARGIN).unwrap();

        present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.player.on_input(keycode);
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