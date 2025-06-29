use crate::transformations::transformations2d::Transformation2D;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
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
            y: self.y * s,
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

    pub fn apply_transformation(&self, transformation: Transformation2D) -> Vec2 {
        // default transformation + displacement/translation
        let t = transformation.matrix;
        Vec2 {
            x: t[0][0] * self.x + t[0][1] * self.y + t[0][2],
            y: t[1][0] * self.x + t[1][1] * self.y + t[1][2],
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Vec2 {
            x: value[0],
            y: value[1],
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests will be added here
}
