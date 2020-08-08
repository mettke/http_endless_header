//! `http_endless_body` for overflowing http server with an infinite body

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
    clippy::used_underscore_binding,
    clippy::exit
)]

use common::{
    env::{setup_env, Env},
    http, run_async,
    tcp::connect,
    write, AsyncWriteExt, Result,
};
use std::{cmp::max, process::exit};

const FRAME_SIZE: usize = 1024;

fn main() -> Result<()> {
    let exit_value = run_async(run())?;
    exit(exit_value);
}

async fn run() -> Result<i32> {
    setup_env()?;
    let mut code = 0;
    let env = Env::new()?;
    code = max(code, content_length_smaller(&env).await?);
    code = max(code, content_length_insane(&env).await?);
    Ok(code)
}

async fn content_length_smaller(env: &Env) -> Result<i32> {
    let mut stream = connect(&env.fqdn_with_port, env.encrypted, true).await?;
    http::write_message(&mut stream, &env.url_returning_200).await?;
    http::write_message(&mut stream, &env.fqdn_with_port).await?;
    http::write_user_agent(&mut stream).await?;
    http::write_content_length(&mut stream, 2).await?;
    http::write_header_end(&mut stream).await?;

    let size = write_attack_body(&mut stream).await?;
    match size {
        // 2^20
        Some(total) if total <= 0x0010_0000 => {
            println!("Wrote {} bytes. This looks like a good limit!", total);
            Ok(0)
        }
        Some(total) => {
            println!(
                "Wrote {} bytes. Either you do not have a limit or its very high. You may want to set it to 1_048_576b or lower!",
                total
            );
            Ok(1)
        }
        None => {
            println!("Aborting as we reached a value outside the usize range while sending data. You may want to introduce a limit to your body parsing!");
            Ok(2)
        }
    }
}

async fn content_length_insane(env: &Env) -> Result<i32> {
    let mut stream = connect(&env.fqdn_with_port, env.encrypted, true).await?;
    http::write_message(&mut stream, &env.url_returning_200).await?;
    http::write_message(&mut stream, &env.fqdn_with_port).await?;
    http::write_user_agent(&mut stream).await?;
    http::write_content_length(&mut stream, usize::max_value()).await?;
    http::write_header_end(&mut stream).await?;

    let size = write_attack_body(&mut stream).await?;
    match size {
        // 2^20
        Some(total) if total <= 0x0010_0000 => {
            println!("Wrote {} bytes. This looks like a good limit!", total);
            Ok(0)
        }
        Some(total) => {
            println!(
                "Wrote {} bytes. Either you do not have a limit or its very high. You may want to set it to 1_048_576b or lower!",
                total
            );
            Ok(1)
        }
        None => {
            println!("Aborting as we reached a value outside the usize range while sending data. You may want to introduce a limit to your body parsing!");
            Ok(2)
        }
    }
}

async fn write_attack_body<S: AsyncWriteExt + Unpin>(stream: &mut S) -> Result<Option<usize>> {
    let buffer = &[0; FRAME_SIZE];
    let mut counter: usize = 0;
    loop {
        if write(stream, buffer).await.is_err() {
            break;
        }
        if let Some(c) = counter.checked_add(FRAME_SIZE) {
            counter = c;
        } else {
            return Ok(None);
        }
    }
    Ok(Some(counter))
}
