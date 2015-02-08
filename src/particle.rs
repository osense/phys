use vec3::Vec3;

#[derive(Copy)]
pub struct Particle {
    pub r: Vec3, // displacement vector
    pub v: Vec3, // velocity vector
    pub t: f64 // time
}

impl Particle {
    pub fn new(displacement: Vec3) -> Particle {
        Particle {
            r: displacement,
            v: Vec3::new(0f64, 0f64, 0f64),
            t: 0f64
        }
    }

    pub fn euler_step(self, a: fn(Particle) -> Vec3, dt: f64) -> Particle {
        Particle {
            r: self.r + self.v * dt,
            v: self.v + a(self) * dt,
            t: self.t + dt
        }
    }
}
