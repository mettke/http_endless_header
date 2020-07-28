mod cert_allow_all;
mod maybe_https_stream;

pub use self::maybe_https_stream::MaybeHttpsStream;

use crate::{tcp::cert_allow_all::CertAllowAll, Context, Result};
use async_std::net::{TcpStream, ToSocketAddrs};
use async_tls::TlsConnector;

/// Connects to a given Server
///
/// # Arguments
///
/// * `server`: Server to connect to
/// * `encrypted`: Whether the connection should be encrypted
/// * `insecure`: Whether a certificate must be valid
///
/// # Errors
/// Fails if the connection to the server could not be established
/// or when the certificate is not valid.
///
#[inline]
pub async fn connect<A: ToSocketAddrs>(
    server: A,
    encrypted: bool,
    insecure: bool,
) -> Result<MaybeHttpsStream> {
    let tcp_stream = TcpStream::connect(server)
        .await
        .context("Unable to connect to server")?;
    if encrypted {
        let connector: TlsConnector = if insecure {
            CertAllowAll::create_connector()
        } else {
            TlsConnector::default()
        };

        Ok(connector
            .connect("www.rust-lang.org", tcp_stream)
            .await
            .context("Unable to establish TLS Connection")?
            .into())
    } else {
        Ok(tcp_stream.into())
    }
}
