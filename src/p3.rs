use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
struct Triangle{
    x: f64,
    y: f64,
    size: f64,
    angle: f64,
    angular_velocity: f64,
    friction: f64,
}

impl Triangle {
    fn new(x: f64, y: f64, size: f64) -> Self {
        Self {
            x,
            y,
            size,
            angle: 0.0,
            angular_velocity: 1.0,
            friction:0.1,
        }
    }
    fn update(&mut self) ->String {
        let cadena: String = String::from("Hola a todos");
        cadena
    }
}

fn main() -> Result<(), String> {
    let mut triangle = Triangle::new(10.0, 20.0, 100.0);
    println!("{}", triangle.update());
    Ok(())
}