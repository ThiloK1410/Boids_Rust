mod boid;

use std::cmp::max;
use std::ops::Neg;
use macroquad::color::{BLACK, WHITE};
use macroquad::input::is_key_down;
use macroquad::input::KeyCode::Escape;
use macroquad::math::Vec2;
use macroquad::prelude::{get_frame_time, screen_width};
use macroquad::window::{clear_background, next_frame, Conf, screen_height};
use quadtree_rs::area::AreaBuilder;
use quadtree_rs::point::Point;
use quadtree_rs::Quadtree;
use crate::boid::Boid;

const BOID_COUNT: usize = 200;

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
    // quadtree saves Boid indices
    let mut quadtree = Quadtree::<i32, i32>::new(
        max(screen_width() as usize, screen_height() as usize).ilog2() as usize);

    let mut boids = Vec::new();
    for _ in 0..BOID_COUNT {
        boids.push(Boid::new());
    }

    let mut lag = 0f32;
    loop {

        if is_key_down(Escape) { break }

        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {
            //updates here
            quadtree.reset();
            for i in 0..BOID_COUNT {
                boids[i].update();
                let (x, y) = (boids[i].get_x(), boids[i].get_y());
                quadtree.insert_pt(Point { x, y }, i as i32);
            }
            let proximity_matrix = get_proximity(&mut boids, &mut quadtree);
            apply_anti_collision_force(&mut boids, &proximity_matrix);

            //---
            lag -= TIME_PER_FRAME;
        }
        //drawing here
        clear_background(BLACK);
        for boid in boids.iter() {
            boid.draw(WHITE);
        }
        boids[0].draw_sensory_range(WHITE);
        //---
        next_frame().await;
    }
}

fn get_proximity(boids: &Vec<Boid>, qt: &mut Quadtree<i32, i32>) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();

    for (i, boid) in boids.iter().enumerate() {
        let mut detected_boids_index: Vec<i32> = Vec::new();

        // creating a query in quadtree for the area around boid with its detection radius
        let rad = boid.get_radius();
        let region = AreaBuilder::default()
            .anchor(boid.get_area_anchor().into())
            .dimensions((rad as i32*2, rad as i32*2).into())
            .build().unwrap();
        let query = qt.query(region);


        //looping over all queried items to check if they are inside boid radius
        for item in query {
            let (index, entry_pos) = (*item.value_ref(),
                                      Vec2::new(item.anchor().x as f32, item.anchor().y as f32));
            let pos = boid.get_pos();
            let distance = pos.distance(entry_pos);
            if distance <= boid.get_radius() && i != index as usize {
                detected_boids_index.push(index);
            }
        }
        //detected_boids_index.remove(i);
        out.push(detected_boids_index);
    }

    return out
}

pub fn apply_anti_collision_force(boids: &mut Vec<Boid>, proximity_matrix: &Vec<Vec<i32>>) {
    for (i, proximity) in proximity_matrix.iter().enumerate() {
        let pos = boids[i].get_pos();

        for j in proximity.iter() {
            let idx = *j as usize;
            let distance = pos.distance(boids[idx].get_pos());
            if distance <= boids[i].get_comfort_zone() {
                let pos2 = Vec2::from(boids[idx].get_pos());
                boids[i].apply_force((pos2-pos).normalize().neg(), 0.05)
            }
        }
    }
}