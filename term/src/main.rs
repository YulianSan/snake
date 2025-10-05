#![allow(warnings)]

use core;
use crossterm::{
    self,
    cursor::{DisableBlinking, Hide, MoveTo, SetCursorStyle, Show},
    event::{poll, read, Event, EventStream, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use futures::{future::FutureExt, select, StreamExt};
use std::{
    io::{stdout, Error, Write},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

const SIZE_GAME: u16 = 20;
const PERFECT_SQUARE: [u16; 2] = [5, 2];

struct DrawGame {
    game: core::Game,
}

impl DrawGame {
    fn draw_background(&self) {
        let background_color = [
            Color::Rgb { r: 0, g: 255, b: 0 },
            Color::Rgb {
                r: 136,
                g: 255,
                b: 136,
            },
        ];

        // execute!(stdout(), Clear(ClearType::Purge)).unwrap();

        for i in 0..SIZE_GAME {
            for j in 0..SIZE_GAME {
                for k in 0..PERFECT_SQUARE[1] {
                    execute!(
                        stdout(),
                        MoveTo(
                            i * PERFECT_SQUARE[0] + PERFECT_SQUARE[0],
                            j * PERFECT_SQUARE[1] + k
                        ),
                        SetBackgroundColor(background_color[((i + j) % 2) as usize]),
                        Print(str::repeat(" ", PERFECT_SQUARE[0] as usize)),
                        ResetColor
                    )
                    .unwrap();
                }
            }
        }
    }

    fn draw_snake(&self) {
        for pos in self.game.snake.body.iter() {
            for i in 0..PERFECT_SQUARE[1] {
                execute!(
                    stdout(),
                    MoveTo(
                        pos.0 * PERFECT_SQUARE[0] + PERFECT_SQUARE[0],
                        pos.1 * PERFECT_SQUARE[1] + i
                    ),
                    SetBackgroundColor(Color::Blue),
                    Print(str::repeat(" ", PERFECT_SQUARE[0] as usize)),
                    ResetColor
                )
                .unwrap();
            }
        }
    }

    async fn get_input(&mut self) {
        let mut reader = EventStream::new();

        loop {
            let mut event = reader.next().fuse().await;

            match event {
                Some(Ok(Event::Key(event))) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode();
                        execute!(stdout(), Show, ResetColor).unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        self.game.input(core::Direction::Up);
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        self.game.input(core::Direction::Down);
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        self.game.input(core::Direction::Left);
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        self.game.input(core::Direction::Right);
                    }
                    e => println!("{:?}", e),
                },
                _ => (),
            }
        }
    }

    fn render(&mut self) {}
}

#[tokio::main]
async fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), Clear(ClearType::All), Hide).unwrap();
    let mut draw = DrawGame {
        game: core::Game::new((5, 5), 20, 20),
    };
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut reader = EventStream::new();

        loop {
            let mut event = reader.next().fuse().await;

            let input = match event {
                Some(Ok(Event::Key(event))) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode();
                        execute!(stdout(), Show, ResetColor).unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Up | KeyCode::Char('w') => Some(core::Direction::Up),
                    KeyCode::Down | KeyCode::Char('s') => Some(core::Direction::Down),
                    KeyCode::Left | KeyCode::Char('a') => Some(core::Direction::Left),
                    KeyCode::Right | KeyCode::Char('d') => Some(core::Direction::Right),
                    e => None,
                },
                _ => None,
            };

            if let Some(input) = input {
                tx.send(input).unwrap();
            }
        }
    });

    loop {
        while let Ok(input) = rx.try_recv() {
            draw.game.input(input);
        }

        draw.draw_background();
        draw.draw_snake();
        draw.game.next();
        sleep(Duration::from_millis(200)).await;
    }

    futures::future::pending::<()>().await;
}
