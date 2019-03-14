const DEFAULT_TILE_SIZE: i32 = 128;

pub struct Camera {
    center_x: f32,
    center_y: f32,
    pub zoom: f32,
    window_w: i32,
    window_h: i32,
}

impl Camera {
    pub fn new(window_w: u32, window_h: u32) -> Camera {
        return Camera {
            center_x: 50.0,
            center_y: 50.0,
            zoom: 4.0,
            window_w: window_w as i32,
            window_h: window_h as i32
        };
    }

    pub fn update_window_size(&mut self, w: u32, h: u32) {
        self.window_w = w as i32;
        self.window_h = h as i32;
    }

    pub fn zoom_in(&mut self) {
        if self.zoom > 1.0 {
            self.zoom /= 2.0;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom < 10.0 {
            self.zoom *= 2.0;
        }
    }

    pub fn move_center(&mut self, x: f32, y: f32) {
        self.center_y += y * self.zoom * 0.25;
        self.center_x += x * self.zoom * 0.25;
    }

    pub fn get_tile_size(&self) -> u32 {
        return (DEFAULT_TILE_SIZE as f32 / self.zoom as f32) as u32;
    }

    pub fn game_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let tile_size: f32 = DEFAULT_TILE_SIZE as f32;

        let screen_x = self.window_w as f32 / 2.0 + (x - self.center_x) * (tile_size / self.zoom);
        let screen_y = self.window_h as f32 / 2.0 + ( y - self.center_y) * (tile_size / self.zoom);
        (screen_x, screen_y)
    }

    pub fn screen_to_game(&self, x: i32, y: i32) -> (f32, f32) {
        let game_x: f32 = (
            x as f32 - 
            self.window_w as f32 / 2.0
        ) / (
            DEFAULT_TILE_SIZE as f32 / self.zoom as f32
        ) + self.center_x as f32;

        let game_y: f32 = (
            y as f32 - 
            self.window_h as f32 / 2.0
        ) / (
            DEFAULT_TILE_SIZE as f32 / self.zoom as f32
        ) + self.center_y as f32;

        (game_x, game_y)
    }
}
