use crate::{extract_u32_le, write_u32_le, HEIGHT, NEW_HEIGHT, NEW_WIDTH, WIDTH};
use common::{Context, Result};
use image::{bmp::BMPEncoder, EncodableLayout, ImageBuffer, Pixel, Rgb};
use std::fs;

pub(crate) fn create_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<()> {
    let mut image = create_png(image)?;
    verify_image(&image, WIDTH, HEIGHT);
    modify_width_and_height(&mut image, NEW_WIDTH, NEW_HEIGHT);
    verify_image(&image, NEW_WIDTH, NEW_HEIGHT);
    fs::write("output.bmp", image).context("Unable to write to image file")?;
    Ok(())
}

fn create_png(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::new();
    let mut encoder = BMPEncoder::new(&mut output);
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
    assert_eq!(b"BM", &image[0..2], "BMP Signature is not valid");
    let img_width = extract_u32_le(image, 18);
    let img_height = extract_u32_le(image, 22);
    assert_eq!(width as u32, img_width, "Image width is invalid");
    assert_eq!(height as u32, img_height, "Image height is invalid");
}

fn modify_width_and_height(image: &mut [u8], new_width: u16, new_height: u16) {
    write_u32_le(image, 2, u32::max_value());
    write_u32_le(image, 18, new_width as u32);
    write_u32_le(image, 22, new_height as u32);
}
