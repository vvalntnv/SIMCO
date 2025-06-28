use super::vectors::Vec2;

pub struct Particle2D {
    pos: Vec2,
    vel: Vec2,
    mass: f32, // mass will always be in kilograms
    force_acc: Vec2,
}

impl Particle2D {
    pub fn new(pos: Vec2, start_vel: Vec2, mass: f32) -> Particle2D {
        Particle2D {
            pos,
            vel: start_vel,
            mass,
            force_acc: Vec2::zero(),
        }
    }

    /// Applying force to the particle
    /// We use newton's laws here for force, and that is
    /// F = m.a,
    /// Adding it into the force accumulator, we can accumulate force
    /// This force will reset every tick
    fn apply_force(&mut self, force: Vec2) {
        self.force_acc.add(force);
    }
}
