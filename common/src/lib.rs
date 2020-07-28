//! `http_endless_header` for overflowing http server with an infinite header

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
    trivial_numeric_casts,
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
    clippy::used_underscore_binding
)]

/// Module for environment variable communication
pub mod env;
/// Module for http methods
pub mod http;
/// Module for tcp connections
pub mod tcp;

pub use anyhow::{bail, Context, Result};
pub use futures::io::AsyncWriteExt;

use std::future::Future;

/// Starts the async main
///
/// # Errors
/// Returns the error from the async function
#[inline]
pub fn run_async<R, F: Future<Output = Result<R>>>(future: F) -> Result<R> {
    async_std::task::block_on(future)
}

/// Writes the given buffer to the given stream
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write<S: AsyncWriteExt + Unpin>(stream: &mut S, data: &[u8]) -> Result<()> {
    stream
        .write_all(data)
        .await
        .context("Unable to write Data to stream")
}
