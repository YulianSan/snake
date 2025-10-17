#![allow(warnings)]

use core::{self};
use std::{
    fs::{create_dir_all, exists, remove_dir_all},
    io,
    time::Duration,
};

use image::ImageBuffer;
use tokio::time::sleep;

const BACKGROUND_COLOR: [[u8; 3]; 2] = [[0u8, 255u8, 0u8], [136, 255, 136]];
const SNAKE_COLOR: [u8; 3] = [0u8, 0u8, 255u8];
const FOOD_COLOR: [u8; 3] = [255u8, 0u8, 0u8];
const WIDTH: usize = 13;
const HEIGHT: usize = 8;
const IMAGE_SIZE: u32 = 100;
const PATH: &str = "/home/sandev/game_snake";

struct DrawGame {
    pub game: core::Game,
}

impl DrawGame {
    pub fn new() -> DrawGame {
        if let Ok(true) = exists(PATH) {
            let _ = remove_dir_all(PATH);
        }

        create_dir_all(PATH).unwrap();

        DrawGame {
            game: core::Game::new((3, 3), WIDTH as u16, HEIGHT as u16),
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
}

#[tokio::main]
async fn main() {
    let mut draw = DrawGame::new();

    loop {
        draw.draw_background();
        draw.draw_snake();
        draw.game.next();
        draw.draw_food();

        sleep(Duration::from_millis(5000)).await;
    }
}
