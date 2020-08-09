use crate::{extract_u16, write_u16, HEIGHT, NEW_HEIGHT, NEW_WIDTH, WIDTH};
use common::{bail, Context, Result};
use image::{jpeg::JPEGEncoder, EncodableLayout, ImageBuffer, Pixel, Rgb};
use std::fs;

pub(crate) fn create_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<()> {
    let mut image = create_jpeg(image)?;
    verify_image(&image, WIDTH, HEIGHT)?;
    modify_width_and_height(&mut image, NEW_WIDTH, NEW_HEIGHT)?;
    verify_image(&image, NEW_WIDTH, NEW_HEIGHT)?;
    fs::write("output.jpeg", image).context("Unable to write to image file")?;
    Ok(())
}

fn create_jpeg(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::new();
    let mut encoder = JPEGEncoder::new(&mut output);
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

fn verify_image(image: &[u8], width: u16, height: u16) -> Result<()> {
    assert_eq!([0xff, 0xd8], &image[0..2], "JPEG SOI is not valid");
    let pos = find_sof0(image)?;
    let img_width = extract_u16(image, pos + 5);
    let img_height = extract_u16(image, pos + 7);
    assert_eq!(width, img_width, "Image width is invalid");
    assert_eq!(height, img_height, "Image height is invalid");
    Ok(())
}

fn find_sof0(image: &[u8]) -> Result<usize> {
    let mut pos = 2;
    loop {
        if pos + 4 > image.len() {
            bail!("Unable to find SOF0 Frame");
        }
        if image[pos..pos + 2] == [0xff, 0xc0] {
            break;
        }
        pos += 2;
        pos += extract_u16(image, pos) as usize;
    }
    Ok(pos)
}

fn modify_width_and_height(image: &mut [u8], new_width: u16, new_height: u16) -> Result<()> {
    let pos = find_sof0(image)?;
    write_u16(image, pos + 5, new_width);
    write_u16(image, pos + 7, new_height);
    Ok(())
}
