#![allow(warnings)]

use core::{self, Direction};
use std::{
    fs::{create_dir_all, exists, remove_dir_all},
    io,
    time::Duration,
};

use image::ImageBuffer;
use rdev::{
    listen, Event,
    EventType::KeyPress,
    Key::{self, DownArrow, LeftArrow, RightArrow, UpArrow},
};
use tokio::{fs::rename, sync::mpsc, time::sleep};

const BACKGROUND_COLOR: [[u8; 3]; 2] = [[0u8, 255u8, 0u8], [136, 255, 136]];
const SNAKE_COLOR: [u8; 3] = [0u8, 0u8, 255u8];
const FOOD_COLOR: [u8; 3] = [255u8, 0u8, 0u8];
const WIDTH: usize = 9;
const HEIGHT: usize = 6;
const IMAGE_SIZE: u32 = 100;
const PATH: &str = "/home/sandev/game_snake";

struct DrawGame {
    pub game: core::Game,
    old_pos: (u16, u16),
}

impl DrawGame {
    pub fn new() -> DrawGame {
        if let Ok(true) = exists(PATH) {
            let _ = remove_dir_all(PATH);
        }

        create_dir_all(PATH).unwrap();

        DrawGame {
            game: core::Game::new((3, 3), WIDTH as u16, HEIGHT as u16),
            old_pos: (3, 3),
        }
    }

    pub fn draw_background(&self) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                DrawGame::create_image(BACKGROUND_COLOR[(i + j) % 2], (i as u16, j as u16));
            }
        }
    }

    pub fn draw_snake(&self) {
        for pos in &self.game.snake.body {
            DrawGame::create_image(SNAKE_COLOR, *pos);
        }
    }

    pub fn draw_food(&self) {
        for core::Food { pos } in self.game.food.iter() {
            DrawGame::create_image(FOOD_COLOR, *pos);
        }
    }

    fn create_image(color: [u8; 3], pos: (u16, u16)) {
        ImageBuffer::from_pixel(IMAGE_SIZE, IMAGE_SIZE, image::Rgb(color))
            .save(format!("{}/{}.png", PATH, pos.0 + (pos.1 * WIDTH as u16)))
            .unwrap();
    }

    fn before_next(&mut self) {
        self.old_pos = self.game.snake.body[0];
    }

    fn after_next(&mut self) {
        let new_pos = self.game.snake.body.last().unwrap();

        DrawGame::create_image(SNAKE_COLOR, *new_pos);
        DrawGame::create_image(
            BACKGROUND_COLOR[(self.old_pos.0 + self.old_pos.1) as usize % 2],
            self.old_pos,
        );
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut draw = DrawGame::new();

    tokio::spawn(async move {
        if let Err(e) = listen(move |event| match event.event_type {
            KeyPress(key) => {
                tx.send(key);
            }
            _ => (),
        }) {
            println!("Error: {:?}", e);
        }
    });

    draw.draw_background();
    draw.draw_snake();

    loop {
        while let Ok(event) = rx.try_recv() {
            match event {
                DownArrow => draw.game.input(Direction::Down),
                UpArrow => draw.game.input(Direction::Up),
                LeftArrow => draw.game.input(Direction::Left),
                RightArrow => draw.game.input(Direction::Right),
                Key::KeyR => draw.game.reset(),
                Key::KeyQ => std::process::exit(0),
                _ => (),
            }
        }
        draw.before_next();
        draw.game.next();

        draw.after_next();
        draw.draw_food();

        sleep(Duration::from_millis(5000)).await;
    }
}
