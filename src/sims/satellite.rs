use vec3::Vec3;
use particle::Particle;
use std::num::Float;


pub fn acc_f(particle: Particle) -> Vec3 {
    let r = particle.r;
    //let u = (-r).normalize();
    //(6.67e-11 * 5.98e24) / (r.magnitude().powi(2i32))
    r.normalize()
}