extern crate ggez;
extern crate rand;

mod food;
mod player;
mod util;

use ggez::*;
use ggez::event::*;
use std::time::Duration;
use food::Food;
use player::Player;
use util::rect;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 590.0;
const GAME_MARGIN: f32 = 16.0;
const GAME_WIDTH: f32 = player::SIZE * 24.0;
const GAME_HEIGHT: f32 = player::SIZE * 22.0;

struct Assets {
    font: graphics::Font
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            font: graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?
        })
    }
}

struct MainState {
    player: Player,
    food: Food,
    assets: Assets,
    score_display: graphics::Text
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let score_display = graphics::Text::new(ctx, "Score: 0", &assets.font)?;
        Ok(MainState {
            player: Player::new(GAME_WIDTH, GAME_HEIGHT),
            food: Food {
                x: 2.0 * player::SIZE,
                y: 2.0 * player::SIZE
            },
            assets: assets,
            score_display: score_display
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        // See if the player ate any food
        if self.player.update(dt, &self.food) {
            // Update the score
            let score = format!("Score: {}", self.player.get_score());
            self.score_display = graphics::Text::new(ctx, &score, &self.assets.font)?;

            // Move the food
            while self.player.is_colliding(&self.food) {
                self.food.rand_pos(24, 22, player::SIZE);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use graphics::*;

        let border_color = Color::new(0.2, 0.2, 0.2, 1.0);
        let game_board_color = Color::new(0.0, 0.0, 0.0, 1.0);
        let grid_color = Color::new(1.0, 1.0, 1.0, 0.2);
        let score_color = Color::new(1.0, 1.0, 1.0, 1.0);

        // Clear the screen and render the border
        set_background_color(ctx, border_color);
        clear(ctx);

        // Render the game space background
        set_color(ctx, game_board_color)?;
        rectangle(
            ctx,
            DrawMode::Fill,
            rect(GAME_MARGIN, GAME_MARGIN, GAME_WIDTH, GAME_HEIGHT))?;

        // Render the food
        self.food.draw(ctx, GAME_MARGIN).unwrap();

        // Render the player inside of the game space
        self.player.draw(ctx, GAME_MARGIN).unwrap();

        // Render the grid
        set_color(ctx, grid_color)?;
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

        // Draw the score
        set_color(ctx, score_color)?;
        let score_destination = Point::new(SCREEN_WIDTH - (self.score_display.width() / 2) as f32 - GAME_MARGIN,
                                           (self.score_display.height() / 2) as f32 + GAME_MARGIN);
        draw(ctx, &self.score_display, score_destination, 0.0)?;

        present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.player.on_input(keycode);
    }
}

pub fn main() {
    let c = conf::Conf {
        window_title: "Snake".to_string(),
        window_icon: "".to_string(),
        window_height: SCREEN_HEIGHT as u32,
        window_width: SCREEN_WIDTH as u32,
        vsync: true,
        resizable: false
    };
    let ctx = &mut Context::load_from_conf("snake", "ggez", c).unwrap();
    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            }
        }
    }
}