use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Circulo {
    x: f64,
    y: f64,
    radio: f64,
    velocidad_x: f64,
    velocidad_y: f64,
}

impl Circulo {
    fn new(x: f64, y:f64) -> Self {
        Self {
            x,
            y,
            radio: 10.0,
            velocidad_x: 2.0,
            velocidad_y: 10.0,
        }
    }
    fn update(&mut self) {
        self.x += self.velocidad_x;
        self.y += self.velocidad_y;

        // Rebotar en los bordes de la pantalla
        
    }
}

fn main() {
    let mut circ = Circulo::new(20.0, 30.0);
    println!("Cículo mágico");
    for frame in 1..60 {
        circ.update();
        print!(
            "Frame: {:02} | Posición: ({:.2}, {:.2})\n",
            frame, circ.x, circ.y
        );
        std::thread::sleep(Duration::from_millis(16));
    }
}