#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(self, s: f32) {
        Vec2 {
            x: self.x * s,
            y: self.x * s,
        };
    }
}
