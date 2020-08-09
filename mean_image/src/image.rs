use crate::{HEIGHT, WIDTH};
use image::{ImageBuffer, Rgb};
use noise::{Billow, MultiFractal, NoiseFn, Seedable};
use rand::{thread_rng, Rng};

fn create_noise_function() -> Billow {
    let mut rng = thread_rng();

    let seed: u32 = rng.gen();
    let frequency: u8 = rng.gen_range(1, 4);
    let octaves: u8 = rng.gen_range(1, 25);
    let lacunarity: f64 = rng.gen_range(1.0, 2.0);
    let persistence: f64 = rng.gen_range(0.0, 0.5);

    Billow::new()
        .set_seed(seed)
        .set_frequency(frequency as f64)
        .set_octaves(octaves as usize)
        .set_lacunarity(lacunarity as f64)
        .set_persistence(persistence as f64)
}

pub(crate) fn generate_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH as u32, HEIGHT as u32);
    let noise = create_noise_function();

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let nx = w as f64 / WIDTH as f64 - 0.5;
            let ny = h as f64 / HEIGHT as f64 - 0.5;

            let r = noise.get([nx, ny, 0.1]) + 1.0;
            let r = (127.5 * r) as u8;
            let g = noise.get([nx, ny, 0.2]) + 1.0;
            let g = (127.5 * g) as u8;
            let b = noise.get([nx, ny, 0.3]) + 1.0;
            let b = (127.5 * b) as u8;
            image.put_pixel(w as u32, h as u32, Rgb([r, g, b]));
        }
    }
    image
}
