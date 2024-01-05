mod boid;

use std::ops::Neg;
use macroquad::color::{BLACK, RED, WHITE};
use macroquad::input::is_key_down;
use macroquad::input::KeyCode::Escape;
use macroquad::math::Vec2;
use macroquad::prelude::{get_frame_time, screen_width};
use macroquad::time::get_time;
use macroquad::window::{clear_background, next_frame, Conf, screen_height};
use quadtree_rs::area::AreaBuilder;
use quadtree_rs::point::Point;
use quadtree_rs::Quadtree;
use crate::boid::Boid;

const BOID_COUNT: usize = 400;

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
    let qt_depth = match (screen_width(), screen_height()) {
        _ if screen_width() > screen_height() => screen_width().log2().ceil() as usize,
        _ => screen_height().log2().ceil() as usize,
    };
    let mut quadtree = Quadtree::<i32, i32>::new(qt_depth);

    let mut boids = Vec::new();
    for _ in 0..BOID_COUNT {
        boids.push(Boid::new());
    }
    let start_time = get_time();
    let mut start = 0f64;
    let mut draw_time = 0f64;
    let mut collision_time = 0f64;
    let mut quadtree_insert_time = 0f64;
    let mut quadtree_query_time = 0f64;

    let mut lag = 0f32;
    loop {
        clear_background(BLACK);
        for boid in boids.iter() {
            boid.draw(WHITE, 1f32);
        }

        if is_key_down(Escape) { break }

        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {

            //updates here

            start = get_time();
            quadtree.reset();
            for i in 0..BOID_COUNT {
                boids[i].update();
                let (x, y) = (boids[i].get_x(), boids[i].get_y());
                quadtree.insert_pt(Point { x, y }, i as i32);
            }
            quadtree_insert_time += get_time() - start;

            start = get_time();
            let proximity_matrix = get_proximity(&mut boids, &mut quadtree);
            quadtree_query_time += get_time() -start;

            start = get_time();
            apply_anti_collision_force(&mut boids, &proximity_matrix);
            collision_time += get_time() -start;

            //---
            lag -= TIME_PER_FRAME;
        }
        //drawing here
        start = get_time();
        //clear_background(BLACK);
        for boid in boids.iter() {
            //boid.draw(WHITE);
        }
        boids[0].draw_sensory_range(WHITE);
        boids[1].draw_sensory_range(WHITE);
        draw_time += get_time() - start;
        //---
        next_frame().await;
    }
    let end_time = get_time();
    let total_runtime = end_time -start_time;
    println!("Total Runtime: {}", total_runtime);
    println!("Time spent drawing: {}  -  {:.2}%", draw_time, 100f64*draw_time/total_runtime);
    println!("Time spent inserting in quadtree: {}  -  {:.2}%",
             quadtree_insert_time, 100f64*quadtree_insert_time/total_runtime);
    println!("Time spent querying from quadtree: {}  -  {:.2}%",
             quadtree_query_time, 100f64*quadtree_query_time/total_runtime);
    println!("Time spent in collision: {}  -  {:.2}%", collision_time, 100f64*collision_time/total_runtime);


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

        if i==0 || i==1 {
            for j in query.clone() {
                let index = *j.value_ref() as usize;
                boids[index].draw(RED, 2f32);
            }
        }


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