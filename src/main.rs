use getch_rs::{Getch, Key};
mod block;
mod game;
use game::*;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));
    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(1000));
            let mut game = game.lock().unwrap();
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };

            if !is_collision(&game.field, &new_pos, &game.block) {
                game.pos = new_pos;
            } else {
                fix_block(&mut game);
                erase_line(&mut game.field);
                if spawn_block(&mut game).is_err() {
                    gameover(&game)
                }
            }
            draw(&game);
        });
    }

    loop {
        let g = Getch::new();
        match g.getch() {
            Ok(Key::Char('q')) => {
                quit();
            }
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            _ => (),
        }
    }
}
