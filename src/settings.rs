use image::Rgba;
use std::f64::consts::PI;

// window settings
pub const WIDTH: f64 = 500.;
pub const HEIGHT: f64 = 500.;

// sim settings
pub const AGENTS: usize = 20_000;
pub const AGENT_COLOR: Rgba<u8> = Rgba::<u8>([150, 0, 150, 50]);
pub const SENSOR_OFFSET_ANGLE: f64 = PI / 8.;
pub const SENSOR_OFFSET_DST: u8 = 15;
pub const SENSOR_R: isize = 2;
pub const TURN_STRENGTH: f64 = PI / 8.;
pub const SPAWN_TYPE: SpawnType = SpawnType::Point;
pub const CIRCLE_ANGLE: f64 = PI; // for circle spawn type, might not be needed

#[allow(dead_code)]
pub enum SpawnType {
    Random,
    Circle,
    Waterfall,
    Point,
    Lines
}
