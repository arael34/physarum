extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

/*
TODO
decide between ImageBuffer or RgbaImage for image
*/

use piston_window::*;
//use piston::event_loop::{Events, EventLoop, EventSettings};
//use piston::input::RenderEvent;
use opengl_graphics::OpenGL;//, Filter, GlGraphics, TextureSettings};
use rand::*;
use rand::distributions::Uniform;
use std::f64::consts::PI;
use ::image::{ImageBuffer, Rgba};//RgbaImage
use ::image::imageops::{blur, brighten};

// window settings
const WIDTH: f64 = 600.;
const HEIGHT: f64 = 600.;

// sim settings
const AGENTS: usize = 100;
const SENSOR_OFFSET_ANGLE: f32 = PI as f32 / 4.;
const TURN_STRENGTH: f64 = PI / 6.;

#[derive(Copy, Clone)]
struct Agent {
    x: f64,
    y: f64,
    ang: f64
}

impl Agent {
    fn new() -> Self { Agent{x: 0., y: 0., ang: 0.} }
    fn update(&mut self) {
        let mut new_x = self.x + self.ang.cos();
        let mut new_y = self.y + self.ang.sin();
        let mut rng = thread_rng();
        if new_x > WIDTH || new_x < 0. {
            new_x = if 
                        if new_x < 0. 
                            {0.} 
                        else 
                            {new_x} 
                    > WIDTH
                        {WIDTH} 
                    else 
                        {new_x};
            self.ang = rng.gen_range(0., 2. * PI);
        }
        if new_y > HEIGHT || new_y < 0. {
            new_y = if 
                        if new_y < 0. 
                            {0.} 
                        else 
                            {new_y} 
                    > HEIGHT
                        {HEIGHT} 
                    else 
                        {new_y};
            self.ang = rng.gen_range(0., 2. * PI);
        }
        self.x = new_x;
        self.y = new_y;
    }
    fn check(&mut self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let weight_forward = self.sense(0., img);
        let weight_right = self.sense(SENSOR_OFFSET_ANGLE, img);
        let weight_left = self.sense(-SENSOR_OFFSET_ANGLE, img);
        if weight_right < weight_forward && weight_forward > weight_left {}
        else if weight_right == weight_left {}
        else if weight_right > weight_left {
            self.ang += TURN_STRENGTH;
        } else {
            self.ang -= TURN_STRENGTH;
        }
    }
    fn sense(&mut self, offset_angle: f32, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> f64 {
        0.
    }
}

struct Simulation {
    agents: [Agent; AGENTS],
}

impl Simulation {
    fn new() -> Self {
        let uniform_x = Uniform::<f64>::new(1., WIDTH);
        let uniform_y = Uniform::<f64>::new(1., HEIGHT);
        let uniform_ang = Uniform::<f64>::new(0., 2. * PI);
        let mut rng = thread_rng();
        let mut agents = [Agent::new(); AGENTS];
        for i in 0..AGENTS {
            agents[i].x = rng.sample(uniform_x);
            agents[i].y = rng.sample(uniform_y);
            agents[i].ang = rng.sample(uniform_ang);
        }
        Simulation{agents}
    }
}

fn main() -> () {
    let mut sim = Simulation::new();

    let opengl = OpenGL::V3_2;
    let mut window:PistonWindow = WindowSettings::new("Simulation", [WIDTH, HEIGHT])
                                  .graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut img = ImageBuffer::new(WIDTH as u32 + 1, HEIGHT as u32 + 1);  

    while let Some(en) = window.next() {
        let texture: G2dTexture = Texture::from_image(&mut window.create_texture_context(), &img, &TextureSettings::new()).unwrap();
        window.draw_2d(&en, |c, g, _d| {
            clear([0., 0., 0., 1.], g);
            for i in 0..AGENTS {
                Agent::check(&mut sim.agents[i], &img);
                Agent::update(&mut sim.agents[i]);
                img.put_pixel(sim.agents[i].x as u32, sim.agents[i].y as u32, Rgba([255, 0, 0, 255]));
            }
            image(&texture, c.transform, g);
            //img = brighten(&img, -5);
            //img = blur(&img, 0.5);
        });
    }
}
