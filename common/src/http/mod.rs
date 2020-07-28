use crate::{write, Result};
use futures::io::AsyncWriteExt;
use std::fmt::Display;

/// Writes the http message using the given url
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write_message<S: AsyncWriteExt + Unpin, D: Display>(
    stream: &mut S,
    url: &D,
) -> Result<()> {
    write(stream, format!("GET {} HTTP/1.0\n", url).as_bytes()).await
}

/// Writes the http host header using the given fqdn and its port
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write_host<S: AsyncWriteExt + Unpin, D: Display>(
    stream: &mut S,
    fqdn_with_port: &D,
) -> Result<()> {
    write(stream, format!("Host: {}\n", fqdn_with_port).as_bytes()).await
}

/// Writes the http user agent
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write_user_agent<S: AsyncWriteExt + Unpin>(stream: &mut S) -> Result<()> {
    write(stream, b"User-Agent: http_endless_header\n").await
}
