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
                y: game.pos.y.checked_add(1).unwrap_or_else(|| game.pos.y),
            };

            if !is_collision(&game.field, &new_pos, game.block) {
                game.pos = new_pos;
            } else {
                fix_block(&mut game);
                erase_line(&mut game.field);
                game.pos = Position::init();
                game.block = rand::random();
            }
            draw(&game);
        });
    }

    loop {
        let g = Getch::new();
        match g.getch() {
            Ok(Key::Char('q')) => {
                // カーソルを再表示
                println!("\x1b[?25h");
                return;
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
                    x: game.pos.x.checked_add(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y.checked_add(1).unwrap_or_else(|| game.pos.y),
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            _ => (),
        }
    }
}