use std::sync::Mutex;

use point::Point;
use glue::gen_rand;


lazy_static! {
    static ref state: Mutex<AppState> = Mutex::new(AppState::new());
}


struct Snow {
    is_alive: bool,
    loc: Point,
    vel: Point,
}

impl Snow {
    fn new() -> Self {
        Snow {
            is_alive: true,
            loc: Point::new(0.0, 0.0),
            vel: Point::new(0.0, 0.0),
        }
    }
}

struct AppState {
    width: usize,
    height: usize,
    snows: Vec<Snow>,
    snow_vel: Point,
    snow_rate: usize,
}

impl AppState {
    fn new() -> Self {
        AppState {
            width: 0,
            height: 0,
            snows: Vec::with_capacity(2048),
            snow_vel: Point::new(0.1, 0.6),
            snow_rate: 1,
        }
    }
}

impl AppState {
    fn init(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    
        let n_snow = (width / 32) * (height / 32);
        for _ in 0..n_snow {
            let x = gen_rand() * width as f32;
            let y = gen_rand() * height as f32;
            
            let mut snow = Snow::new();
            snow.loc.set(x, y);
            snow.vel = self.snow_vel;
            
            self.snows.push(snow);
        }
    }

    fn update(&mut self, buf: &mut [u8]) {
        for snow in &mut self.snows {
            let (x, y) = snow.loc.floor();
            let offset = ((y as usize) * self.width + (x as usize)) * 4;
            
            let bottom_offset = offset + self.width * 4;
            
            if bottom_offset >= buf.len() || buf[bottom_offset] == 254 {
                // Accumulate snow
                buf[offset + 0] = 254;
                buf[offset + 1] = 254;
                buf[offset + 2] = 254;
                buf[offset + 3] = 255;

                snow.is_alive = false;
            }
            else {
                // Clear old pixel
                buf[offset + 0] = 0;
                buf[offset + 1] = 0;
                buf[offset + 2] = 0;
                buf[offset + 3] = 255;
                
                // Move snow
                snow.loc += snow.vel;
                
                // Check bound
                let (x, _y) = snow.loc.floor();
                if x < 0 {
                    snow.loc.x += self.width as f32;
                }
                else if (x as usize) >= self.width {
                    snow.loc.x -= self.width as f32;
                }
            }
        }
        
        // Remove dead snows
        self.snows.retain(|snow| snow.is_alive);
        
        // Create snows
        for _ in 0..self.snow_rate {
            let x = gen_rand() * self.width as f32;
            
            let mut snow = Snow::new();
            snow.loc.set(x, 0.0);
            snow.vel = self.snow_vel;
            
            self.snows.push(snow);
        }
    }
    
    fn render(&self, buf: &mut [u8]) {
        for snow in &self.snows {
            let (x, y) = snow.loc.floor();
            
            if x >= 0 && (x as usize) < self.width
                && y >= 0 && (y as usize) < self.height {
            
                let offset = ((y as usize) * self.width + (x as usize)) * 4;
                
                // RGBA
                buf[offset + 0] = 255;
                buf[offset + 1] = 255;
                buf[offset + 2] = 255;
                buf[offset + 3] = 255;
            }
        }
    }
}


pub fn init(width: usize, height: usize) {
    state.lock().unwrap()
        .init(width, height);
}

pub fn update(buf: &mut [u8]) {
    state.lock().unwrap()
        .update(buf);
}

pub fn render(buf: &mut [u8]) {
    state.lock().unwrap()
        .render(buf);
}