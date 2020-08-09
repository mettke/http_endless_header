use crate::{extract_u32, write_u32, HEIGHT, NEW_HEIGHT, NEW_WIDTH, WIDTH};
use common::{Context, Result};
use crc::{crc32, Hasher32};
use image::{png::PNGEncoder, EncodableLayout, ImageBuffer, Pixel, Rgb};
use std::fs;

pub(crate) fn create_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<()> {
    let mut image = create_png(image)?;
    verify_image(&image, WIDTH, HEIGHT);
    modify_width_and_height(&mut image, NEW_WIDTH, NEW_HEIGHT);
    verify_image(&image, NEW_WIDTH, NEW_HEIGHT);
    fs::write("output.png", image).context("Unable to write to image file")?;
    Ok(())
}

fn create_png(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Vec<u8>> {
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

fn verify_image(image: &[u8], width: u16, height: u16) {
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
    assert_eq!(width as u32, img_width, "Image width is invalid");
    assert_eq!(height as u32, img_height, "Image height is invalid");

    let data_crc = compute_checksum(image, 12, chunk_length as usize + 4);
    let crc = extract_u32(image, 29);
    assert_eq!(data_crc, crc, "CRC checksum does not match");
}

fn compute_checksum(data: &[u8], start: usize, length: usize) -> u32 {
    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(&data[start..start + length]);
    digest.sum32()
}

fn modify_width_and_height(image: &mut [u8], new_width: u16, new_height: u16) {
    write_u32(image, 16, new_width as u32);
    write_u32(image, 20, new_height as u32);

    let chunk_length = extract_u32(image, 8);
    let data_crc = compute_checksum(image, 12, chunk_length as usize + 4);
    write_u32(image, 29, data_crc);
}
