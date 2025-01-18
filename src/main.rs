use med_ray_caster_lib::start;

pub fn main() {
    tracing_subscriber::fmt::init();
    let _ = start();
}
