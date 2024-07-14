use bevy_color::Color;
use rand::Rng;

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::srgb(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}
