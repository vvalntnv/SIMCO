use std::intrinsics::sqrtf32;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(self, s: f32) -> Vec2 {
        Vec2 {
            x: self.x * s,
            y: self.x * s,
        }
    }

    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot_prod(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // proj_onto_vector-self
    pub fn project_to(self, onto_vector: Vec2) -> Vec2 {
        let dot_product = self.dot_prod(onto_vector);
        let magnitude = onto_vector.magnitude().abs();
        let c = dot_product / magnitude.powi(2);

        onto_vector.scale(c)
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }
}
