use rand::{Rng};
use std::collections::HashMap;


// Inspired by https://en.wikipedia.org/wiki/Perlin_noise

pub struct PerlinNoise {
    zoom_level: u32,
    gradients: HashMap<(i32, i32), (f32, f32)>
}

impl PerlinNoise {
    pub fn new(zoom_level: u32) -> PerlinNoise {
        PerlinNoise {
            zoom_level: zoom_level,
            gradients: HashMap::new()
        }
    }

    fn lerp(&self, a0: f32, a1: f32, w: f32) -> f32 {
        return (1.0 - w) * a0 + w * a1;
    }

    fn grad_at(&mut self, x: i32, y: i32) -> (f32, f32) {
        match self.gradients.get(&(x, y)) {
            Some(gradient) => {
                return *gradient;
            }
            _ => {
                let mut randomizer = rand::thread_rng();
                let x_value: f32 = randomizer.gen_range(-1.0, 1.0);
                let y_value: f32 = randomizer.gen_range(-1.0, 1.0);
                self.gradients.insert((x,y), (x_value, y_value));
                return (x_value, y_value);
            }
        }
    }

    fn dot_grid_gradient(&mut self, ix: i32, iy: i32, x: f32, y: f32) -> f32 {
        let dx: f32 = x - ix as f32;
        let dy: f32 = y - iy as f32;

        let gradient = self.grad_at(ix, iy);

        return dx * gradient.1 + dy * gradient.0;
    }

    pub fn value_at(&mut self, x: i32, y: i32) -> f32 {
        let fx: f32 = x as f32 / self.zoom_level as f32;
        let fy: f32 = y as f32 / self.zoom_level as f32;

        let x0 = fx as i32;
        let x1 = fx as i32 + 1;
        let y0 = fy as i32;
        let y1 = fy as i32 + 1;

        let sx = fx - x0 as f32;
        let sy = fy - y0 as f32;

        let dgg1 = self.dot_grid_gradient(x0, y0, fx, fy);
        let dgg2 = self.dot_grid_gradient(x1, y0, fx, fy);
        let dgg3 = self.dot_grid_gradient(x0, y1, fx, fy);
        let dgg4 = self.dot_grid_gradient(x1, y1, fx, fy);

        return self.lerp(
            self.lerp(dgg1, dgg2, sx),
            self.lerp(dgg3, dgg4, sx),
            sy
        )
    }
}



