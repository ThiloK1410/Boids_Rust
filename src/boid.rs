use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::draw_triangle_lines;
use macroquad::shapes::{draw_circle_lines};
use macroquad::window::{screen_height, screen_width};
use rand::random;

pub struct Boid {
    pos: Vec2,
    dir: Vec2,
    vel: f32,
    detection_radius: f32,
    comfort_zone: f32,
}

impl Boid {
    pub fn new() -> Boid {
        Boid {
            pos: Vec2::new(random::<f32>()*screen_width(), random::<f32>()*screen_height()),
            dir: Vec2::new(random::<f32>()-0.5f32, random::<f32>()-0.5f32).normalize(),
            vel: 1.0,
            detection_radius: 100f32,
            comfort_zone: 20f32,
        }
    }
    pub fn draw(&self, color: Color) {
        let perp = self.dir.perp().normalize();
        let (a, b, c) = (self.pos-self.dir*4f32-perp*4f32,
                         self.pos-self.dir*4f32+perp*4f32,
                         self.pos+self.dir*8f32);
        draw_triangle_lines(a, b, c, 1f32, color);
    }
    pub fn draw_sensory_range(&self, color: Color) {
        draw_circle_lines(self.pos.x, self.pos.y, self.detection_radius, 1f32, color);
        draw_circle_lines(self.pos.x, self.pos.y, self.comfort_zone, 1f32, color);
    }
    pub fn update(&mut self) {
        self.pos += self.dir * self.vel;
        let puffer: f32 = 20f32;
        match (self.pos.x, self.pos.y) {
            (w, _) if w <= 0f32 -puffer => self.pos.x = screen_width()+puffer,
            (w, _) if w >= screen_width()+puffer => self.pos.x = 0f32-puffer,
            (_, h) if h <= 0f32-puffer => self.pos.y = screen_height()+puffer,
            (_, h) if h >= screen_height()+puffer => self.pos.y = 0f32-puffer,
            _ => ()
        }
    }
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
    pub fn get_x(&self) -> i32 {
        return self.pos.x as i32
    }
    pub fn get_y(&self) -> i32 {
        return self.pos.y as i32
    }
    pub fn get_area_anchor(&self) -> (i32, i32) {
        (self.get_x()-self.detection_radius as i32, self.get_y()-self.detection_radius as i32)
    }
    pub fn get_radius(&self) -> f32 {
        self.detection_radius
    }
    pub fn get_comfort_zone(&self) -> f32 {
        self.comfort_zone
    }
    pub fn apply_force(&mut self, unit_direction: Vec2, factor: f32) {
        self.dir = (self.dir + unit_direction * factor).normalize();
    }
}