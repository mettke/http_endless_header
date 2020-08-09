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

mod args;
mod gif;
mod image;
mod jpeg;
mod png;

use args::{Args, ImageFormat};
use clap::derive::Clap;
use common::Result;

pub(crate) const WIDTH: u16 = 512;
pub(crate) const HEIGHT: u16 = 512;

// Takes about 12GB of RAM to load
pub(crate) const NEW_WIDTH: u16 = 65_500;
pub(crate) const NEW_HEIGHT: u16 = 65_500;

pub(crate) fn extract_u32(data: &[u8], start: usize) -> u32 {
    let mut buf = [0_u8; 4];
    buf.copy_from_slice(&data[start..start + 4]);
    u32::from_be_bytes(buf)
}

pub(crate) fn extract_u16(data: &[u8], start: usize) -> u16 {
    let mut buf = [0_u8; 2];
    buf.copy_from_slice(&data[start..start + 2]);
    u16::from_be_bytes(buf)
}

pub(crate) fn extract_u16_le(data: &[u8], start: usize) -> u16 {
    let mut buf = [0_u8; 2];
    buf.copy_from_slice(&data[start..start + 2]);
    u16::from_le_bytes(buf)
}

pub(crate) fn write_u32(data: &mut [u8], start: usize, val: u32) {
    let bytes = val.to_be_bytes();
    let buf = &mut data[start..start + 4];
    buf.copy_from_slice(&bytes);
}

pub(crate) fn write_u16(data: &mut [u8], start: usize, val: u16) {
    let bytes = val.to_be_bytes();
    let buf = &mut data[start..start + 2];
    buf.copy_from_slice(&bytes);
}

pub(crate) fn write_u16_le(data: &mut [u8], start: usize, val: u16) {
    let bytes = val.to_le_bytes();
    let buf = &mut data[start..start + 2];
    buf.copy_from_slice(&bytes);
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let image = image::generate_image();
    match args.format {
        ImageFormat::JPEG => jpeg::create_image(&image),
        ImageFormat::PNG => png::create_image(&image),
        ImageFormat::GIF => gif::create_image(&image),
    }
}

#[allow(clippy::use_debug, dead_code)]
fn print_dimension(args: &Args) {
    let d = match args.format {
        ImageFormat::JPEG => ::image::image_dimensions("./output.jpeg"),
        ImageFormat::PNG => ::image::image_dimensions("./output.png"),
        ImageFormat::GIF => ::image::image_dimensions("./output.gif"),
    };
    println!("{:?}", d);
}

#[allow(clippy::use_debug, dead_code, clippy::result_expect_used)]
fn open(args: &Args) {
    let _ = match args.format {
        ImageFormat::JPEG => ::image::open("./output.jpeg"),
        ImageFormat::PNG => ::image::open("./output.png"),
        ImageFormat::GIF => ::image::open("./output.gif"),
    }
    .expect("Unable to open file");
}
