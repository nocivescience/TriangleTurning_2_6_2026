use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Triangle {
    x: f64,
    y: f64,
    size: f64,
    angle: f64,
    angular_velocity: f64,
    friction: f64,
}

impl Triangle {
    fn new (x: f64, y: f64, size: f64) -> Self {
        Self {
            x,
            y,
            size,
            angle: 0.0,
            angular_velocity: 0.0,
            friction: 0.98,
        }
    }
    fn update(&mut self) {
        if self.angular_velocity.abs() > 0.01 {
            self.angle += self.angular_velocity;
            self.angular_velocity *= self.friction;
        } else {
            self.angular_velocity = 0.0;
        }
    }
}

fn main() {
    let mut triangle = Triangle::new(
        (SCREEN_WIDTH/2) as f64,
        (SCREEN_HEIGHT/2) as f64,
        100.0,
    );
    triangle.angular_velocity = 0.5; // Velocidad inicial para probar la fricción
    println!("--- INICIANDO SIMULACIÓN ---");
    for frame in 1..60 {
        triangle.update();
        println!(
            "Frame: {:02} | Ángulo: {:.4} | Vel. Angular: {:.4}",
            frame, triangle.angle, triangle.angular_velocity
        )
    }
}