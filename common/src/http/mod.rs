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
    write(stream, b"User-Agent: sec_tool_belt\n").await
}

/// Writes the http user agent
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write_content_length<S: AsyncWriteExt + Unpin>(stream: &mut S, length: usize) -> Result<()> {
    write(stream, format!("Content-Length: {}\n", length).as_bytes()).await
}

/// Ends the http header
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write_header_end<S: AsyncWriteExt + Unpin>(stream: &mut S) -> Result<()> {
    write(stream, b"\n").await
}

/// Writes body data
///
/// # Errors
/// Fails if the OS is unable to write data to the given stream
#[inline]
pub async fn write_body<S: AsyncWriteExt + Unpin, B: AsRef<[u8]>>(
    stream: &mut S,
    data: B,
) -> Result<()> {
    let data: &[u8] = data.as_ref();
    write(stream, data).await
}
