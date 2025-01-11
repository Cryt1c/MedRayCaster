use med_ray_caster::start;

pub fn main() {
    tracing_subscriber::fmt::init();
    let _ = start();
}
