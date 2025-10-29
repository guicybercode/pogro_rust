mod consts;
mod effects;
mod paddle;
mod ball;
mod menu;
mod game;
mod audio;

use macroquad::prelude::*;
use menu::{Menu, MenuChoice};
use game::Game;

#[macroquad::main("Colorful Pong")]
async fn main() {
    let mut current_state = GameState::Menu;
    let mut menu = Menu::new();
    let mut game: Option<Game> = None;

    loop {
        let dt = get_frame_time();

        match current_state {
            GameState::Menu => {
                clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
                match menu.update(dt) {
                    MenuChoice::OnePlayer => {
                        game = Some(Game::new(false));
                        current_state = GameState::Playing;
                    }
                    MenuChoice::TwoPlayers => {
                        game = Some(Game::new(true));
                        current_state = GameState::Playing;
                    }
                    MenuChoice::None => {}
                }
                menu.draw();
            }
            GameState::Playing => {
                if let Some(ref mut game_instance) = game {
                    if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                        current_state = GameState::Paused;
                    } else {
                        clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
                        let result = game_instance.update(dt);
                        game_instance.draw();
                        
                        match result {
                            game::GameResult::Continue => {}
                            game::GameResult::LeftWins => {
                                current_state = GameState::GameOver(true);
                            }
                            game::GameResult::RightWins => {
                                current_state = GameState::GameOver(false);
                            }
                        }
                    }
                }
            }
            GameState::Paused => {
                if let Some(ref mut game_instance) = game {
                    clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
                    game_instance.draw();
                    game_instance.draw_pause_screen();
                    
                    if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                        current_state = GameState::Playing;
                    }
                }
            }
            GameState::GameOver(left_won) => {
                if let Some(ref mut game_instance) = game {
                    clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
                    game_instance.draw();
                    game_instance.draw_win_screen(left_won);
                    
                    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                        current_state = GameState::Menu;
                        game = None;
                        menu = Menu::new();
                    }
                }
            }
        }

        next_frame().await;
    }
}

enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver(bool),
}

