use crate::{extract_u16_le, write_u16_le, HEIGHT, NEW_HEIGHT, NEW_WIDTH, WIDTH};
use common::{bail, Context, Result};
use image::{gif::Encoder, EncodableLayout, ImageBuffer, Pixel, Rgb};
use std::fs;

pub(crate) fn create_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<()> {
    let mut image = create_gif(image)?;
    verify_image(&image, WIDTH, HEIGHT)?;
    modify_width_and_height(&mut image, NEW_WIDTH, NEW_HEIGHT)?;
    verify_image(&image, NEW_WIDTH, NEW_HEIGHT)?;
    fs::write("output.gif", image).context("Unable to write to image file")?;
    Ok(())
}

fn create_gif(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::new();
    let mut encoder = Encoder::new(&mut output);
    encoder
        .encode(
            image.as_bytes(),
            image.width(),
            image.height(),
            Rgb::<u8>::COLOR_TYPE,
        )
        .context("Cannot decode image")?;
    drop(encoder);
    Ok(output)
}

#[allow(clippy::shadow_same)]
fn skip_global_color_table(image: &[u8], pos: usize) -> Result<usize> {
    let mut pos = pos;
    loop {
        if pos > image.len() {
            bail!("Unable to find Local Image Descriptor");
        }
        match image[pos] {
            0x21 | 0x2c => return Ok(pos), // Start of Extension | Local Image Descriptor
            _ => {}
        }
        pos += 3;
    }
}

#[allow(clippy::shadow_same)]
fn skip_extensions(image: &[u8], pos: usize) -> Result<usize> {
    let mut pos = pos;
    loop {
        if pos + 1 > image.len() {
            bail!("Unable to find Local Image Descriptor");
        }
        let introducer = image[pos];
        let label = image[pos + 1];
        match introducer {
            0x21 => match label {
                // Plain Text Extension
                0x01 => pos += 17,
                // Graphics Control Extension
                0xf9 => pos += 8,
                // Comment Extension block
                0xfe => pos += 4,
                // Application Extension block
                0xff => pos += 16,
                _ => bail!("Invalid label"),
            },
            0x2c => return Ok(pos),
            _ => bail!("Invalid introducer"),
        }
    }
}

#[allow(clippy::panic)]
fn verify_image(image: &[u8], width: u16, height: u16) -> Result<()> {
    assert_eq!(b"GIF", &image[0..3], "GIF Signature is not valid");
    assert!(
        b"87a" == &image[3..6] || b"89a" == &image[3..6],
        "GIF Version is not valid"
    );
    let img_width = extract_u16_le(image, 6);
    let img_height = extract_u16_le(image, 8);
    assert_eq!(width, img_width, "Image width is invalid");
    assert_eq!(height, img_height, "Image height is invalid");

    let pos = skip_global_color_table(image, 13)?;
    let mut pos = skip_extensions(image, pos)?;

    while image[pos] == 0x2c {
        let img_width = extract_u16_le(image, pos + 5);
        let img_height = extract_u16_le(image, pos + 7);
        assert_eq!(width, img_width, "Image width is invalid");
        assert_eq!(height, img_height, "Image height is invalid");
        pos += 10;
    }
    Ok(())
}

fn modify_width_and_height(image: &mut [u8], new_width: u16, new_height: u16) -> Result<()> {
    write_u16_le(image, 6, new_width);
    write_u16_le(image, 8, new_height);

    let pos = skip_global_color_table(image, 13)?;
    let mut pos = skip_extensions(image, pos)?;

    while image[pos] == 0x2c {
        write_u16_le(image, pos + 5, new_width);
        write_u16_le(image, pos + 7, new_height);
        pos += 10;
    }
    Ok(())
}
