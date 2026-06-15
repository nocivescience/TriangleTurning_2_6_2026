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
    fn new(x: f64, y: f64, size: f64) -> Self {
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

    fn is_clicked(&self, mx: i32, my: i32) -> bool {
        let half = self.size / 2.0;
        let left = self.x - half;
        let right = self.x + half;
        let top = self.y - half;
        let bottom = self.y + half;

        (mx as f64) >= left && (mx as f64) <= right && (my as f64) >= top && (my as f64) <= bottom
    }

    fn draw(&self, canvas: &sdl2::render::WindowCanvas) {
        let h = self.size * (3.0_f64.sqrt() / 2.0);
        let r_circum = h * (2.0 / 3.0); 
        let r_in = h * (1.0 / 3.0);     

        let local_vertices = [
            (0.0, -r_circum),            
            (self.size / 2.0, r_in),     
            (-self.size / 2.0, r_in),    
        ];

        let rad = self.angle.to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();

        let mut vx = [0i16; 3];
        let mut vy = [0i16; 3];

        for (i, &(lx, ly)) in local_vertices.iter().enumerate() {
            let rx = lx * cos_a - ly * sin_a;
            let ry = lx * sin_a + ly * cos_a;
            vx[i] = (self.x + rx) as i16;
            vy[i] = (self.y + ry) as i16;
        }

        // Color cambiado a Blanco para máxima visibilidad
        let _ = canvas.filled_polygon(&vx, &vy, Color::RGB(255, 255, 255));
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Triángulo Giratorio en Rust", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // CONFIGURACIÓN CORREGIDA CON ACCELERATED Y VSYNC
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut triangle = Triangle::new(SCREEN_WIDTH as f64 / 2.0, SCREEN_HEIGHT as f64 / 2.0, 180.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown { x, y, .. } => {
                    if triangle.is_clicked(x, y) {
                        triangle.angular_velocity = 30.0; 
                    }
                }
                _ => {}
            }
        }

        triangle.update();

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        triangle.draw(&canvas);

        canvas.present();
        // Al usar present_vsync(), el delay ya lo maneja el monitor,
        // pero dejamos esto como respaldo para evitar consumir 100% de CPU
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}