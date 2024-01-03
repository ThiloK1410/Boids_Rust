mod boid;

use macroquad::color::{BLACK, WHITE};
use macroquad::input::is_key_down;
use macroquad::input::KeyCode::Escape;
use macroquad::prelude::get_frame_time;
use macroquad::window::{clear_background, next_frame, Conf};
use quadtree_rs::point::Point;
use quadtree_rs::Quadtree;
use crate::boid::Boid;

const BOID_COUNT: usize = 100;

const FPS: i32 = 30;    //fps for physics
const TIME_PER_FRAME: f32 = 1f32 / FPS as f32;

fn get_conf() -> Conf {
    Conf {
        window_title: "Boids".to_string(),
        window_width: 0,
        window_height: 0,
        high_dpi: false,
        fullscreen: true,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_conf())]
async fn main() {
    let mut quadtree = Quadtree::<i32, &Boid>::new(4);

    let mut boids = Vec::new();
    for _ in 0..BOID_COUNT {
        boids.push(Boid::new());
    }

    let mut lag = 0f32;
    loop {
        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {
            //updates here
            if is_key_down(Escape) { break }

            for boid in boids.iter() {
                let (x, y) = (boid.get_x(), boid.get_y());
                quadtree.insert_pt(Point { x, y }, boid);
            }

            for boid in boids.iter_mut() {
                boid.update();
            }
            //---
            lag -= TIME_PER_FRAME;
        }
        //drawing here
        clear_background(BLACK);
        for boid in boids.iter() {
            boid.draw(WHITE);
        }
        //---
        next_frame().await;
    }
}
