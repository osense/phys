extern crate phys;
use phys::*;

fn main() {
    let mut state = Particle::new(Vec3::new(1f64, 2f64, 3f64));
    for _ in 0..10 {
        state = state.euler_step(sims::satellite::acc_f, 1f64);
        println!("{}", state.r.string());
    }
}