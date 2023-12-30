extern crate piston_window;
extern crate opengl_graphics;
extern crate rand;
extern crate image;
extern crate rayon;

/*
TODO
clean up types, specifically in sense function
agents seem to stick to walls on the right and bottom sides
*/

use piston_window::*;
use image::{ImageBuffer, Rgba};
use opengl_graphics::OpenGL;
use rand::*;
use rand::distributions::Uniform;
use std::f64::consts::PI;
use rayon::prelude::*;

mod settings;
use settings::*;

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
        if new_x > WIDTH - 1. || new_x < 0. {
            new_x = if 
                if new_x < 0. { 0. } else { new_x } > WIDTH - 1.
                    { WIDTH - 1. } 
                else 
                    {new_x};
            self.ang = rng.gen_range(0.0..2. * PI);
        }
        if new_y > HEIGHT - 1. || new_y < 0. {
            new_y = if 
                if new_y < 0. { 0. } else { new_y } > HEIGHT - 1.
                    {HEIGHT - 1.} 
                else 
                    {new_y};
            self.ang = rng.gen_range(0.0..2. * PI);
        }
        self.x = new_x;
        self.y = new_y;
    }
    fn check(&mut self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let weight_forward = self.sense(0., img);
        let weight_right = self.sense(SENSOR_OFFSET_ANGLE, img);
        let weight_left = self.sense(-SENSOR_OFFSET_ANGLE, img);
        let rng = thread_rng().gen_range(0.0..1.01);
        if weight_right < weight_forward && weight_forward > weight_left {}
        else if weight_right == weight_left {
            self.ang += (rng - 0.5) * 0.2 * TURN_STRENGTH;
        }
        else if weight_right > weight_left {
            self.ang += rng * TURN_STRENGTH;
        } else {
            self.ang -= rng * TURN_STRENGTH;
        }
    }
    fn sense(&mut self, offset_angle: f64, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> f64 {
        let angle = self.ang + offset_angle;
        let center_x = (self.x + angle.cos() * SENSOR_OFFSET_DST as f64) as isize;
        let center_y = (self.y + angle.sin() * SENSOR_OFFSET_DST as f64) as isize;
        let mut sum: f64 = 0.;
        for x in center_x - SENSOR_R..=center_x + SENSOR_R {
            for y in center_y - SENSOR_R..= center_y + SENSOR_R {
                if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    sum += pixel[2] as f64 / 255.;
                    sum += pixel[1] as f64 / 255.;
                }
            }
        }
        sum
    }
}

struct Simulation {
    agents: Vec<Agent>
}

impl Simulation {
    fn new() -> Self {
        let uniform: Uniform<f64> = Uniform::<f64>::new(0., 1.);
        let mut rng = thread_rng();
        let mut agents = vec![Agent::new(); AGENTS];
        match SPAWN_TYPE {
            SpawnType::Random => {
                for agent in agents.iter_mut() {
                    agent.x = rng.sample(uniform) * WIDTH;
                    agent.y = rng.sample(uniform) * HEIGHT;
                    agent.ang = rng.sample(uniform) * 2. * PI;
                }
            }
            SpawnType::Circle => {
                for agent in agents.iter_mut() {
                    let angle = rng.sample(uniform) * 2. * PI;
                    let rad = rng.sample(uniform) * (HEIGHT / 2. - 1.);
                    agent.x = WIDTH / 2. + angle.cos() * rad;
                    agent.y = HEIGHT / 2. + angle.sin() * rad;
                    agent.ang = angle * CIRCLE_ANGLE;
                }
            }
            SpawnType::Waterfall => {
                for agent in agents.iter_mut() {
                    agent.x = rng.sample(uniform) * WIDTH;
                    agent.y = rng.sample(uniform) * HEIGHT;
                    agent.ang = PI / 2.;
                }
            }
            SpawnType::Point => {
                for agent in agents.iter_mut() {
                    agent.x = WIDTH / 2.;
                    agent.y = HEIGHT / 2.;
                    agent.ang = rng.sample(uniform) * 2. * PI;
                }
            }
            _ => {}
        }
        Simulation{agents}
    }
}

fn reduce_pixel(value: u8) -> u8 {
    if value > 3 {
        return value - 3;
    } else if value > 0 {
        return 0;
    }
    0
}

fn main() -> () {
    let mut sim = Simulation::new();

    let opengl = OpenGL::V3_2;
    let mut window:PistonWindow = WindowSettings::new("Simulation", [WIDTH, HEIGHT])
                                  .graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut img = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(60);
    while let Some(en) = events.next(&mut window) {
        let texture: G2dTexture = Texture::from_image(&mut window.create_texture_context(), &img, &TextureSettings::new()).unwrap();
        window.draw_2d(&en, |c, g, _d| {
            sim.agents.par_iter_mut().for_each(|agent| {
                agent.check(&img);
                agent.update();
            });
            for agent in &sim.agents {
                img.put_pixel(agent.x as u32, agent.y as u32, AGENT_COLOR);
            }
            image(&texture, c.transform, g);
            for pixel in img.pixels_mut() {
                pixel[0] = reduce_pixel(pixel[0]); // for different colors
                pixel[1] = reduce_pixel(pixel[1]);
                pixel[2] = reduce_pixel(pixel[2]);
            }
        });
    }
}
