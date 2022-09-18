use druid::{Point, Vec2};

pub struct Viewport {
    pub origin: Point,
    pub scale: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            origin: Default::default(),
            scale: 1.0,
        }
    }
}

impl Viewport {
    pub fn apply_mouse_move(&mut self, mouse_delta: Vec2) {
        self.origin += mouse_delta / self.scale;
    }

    pub fn apply_scale(&mut self, screen_scale_origin: Point, scale_amount: f64) {
        let original_scale = self.scale;
        // Keep scale between 20% and 1000% and scale quadratically
        self.scale = (self.scale + (scale_amount * self.scale)).clamp(0.20, 10.0);
        // Zoom based on the mouse position - translate view to keeps mouse pos in scene the same
        self.origin += screen_scale_origin.to_vec2() * (1.0 / original_scale - 1.0 / self.scale);
    }

    pub fn scene_coord_to_screen(&self, point: Point) -> Point {
        ((point - self.origin) * self.scale).to_point()
    }

    pub fn screen_coord_to_scene(&self, point: Point) -> Point {
        ((point.to_vec2() / self.scale) + self.origin.to_vec2()).to_point()
    }
}

