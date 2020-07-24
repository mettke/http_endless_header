use anyhow::{Context, Result};
use async_std::net::TcpStream;
use async_tls::{client::TlsStream, TlsConnector};
use futures::{
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    task,
};
use rustls::{
    Certificate, ClientConfig, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError,
};
use std::{pin::Pin, sync::Arc};

const SERVER: &str = "127.0.0.1:8443";
const PATH: &str = "/app/";
const ENCRYPTED: bool = true;

#[allow(clippy::large_enum_variant)]
enum MaybeHttpsStream {
    Http(TcpStream),
    Https(TlsStream<TcpStream>),
}

impl From<TcpStream> for MaybeHttpsStream {
    fn from(stream: TcpStream) -> Self {
        MaybeHttpsStream::Http(stream)
    }
}

impl From<TlsStream<TcpStream>> for MaybeHttpsStream {
    fn from(stream: TlsStream<TcpStream>) -> Self {
        MaybeHttpsStream::Https(stream)
    }
}

impl AsyncRead for MaybeHttpsStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<io::Result<usize>> {
        match *self {
            Self::Http(ref mut s) => Pin::new(s).poll_read(cx, buf),
            Self::Https(ref mut s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for MaybeHttpsStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<io::Result<usize>> {
        match *self {
            Self::Http(ref mut s) => Pin::new(s).poll_write(cx, buf),
            Self::Https(ref mut s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<io::Result<()>> {
        match *self {
            Self::Http(ref mut s) => Pin::new(s).poll_flush(cx),
            Self::Https(ref mut s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<io::Result<()>> {
        match *self {
            Self::Http(ref mut s) => Pin::new(s).poll_close(cx),
            Self::Https(ref mut s) => Pin::new(s).poll_close(cx),
        }
    }
}

struct CertAllowAll;

impl ServerCertVerifier for CertAllowAll {
    fn verify_server_cert(
        &self,
        _roots: &RootCertStore,
        _presented_certs: &[Certificate],
        _dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        Ok(ServerCertVerified::assertion())
    }
}

fn main() -> Result<()> {
    async_std::task::block_on(run())
}

async fn run() -> Result<()> {
    let tcp_stream = TcpStream::connect(SERVER)
        .await
        .context("Unable to connect to server")?;
    let mut stream: MaybeHttpsStream = if ENCRYPTED {
        let mut config = ClientConfig::new();
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(CertAllowAll));
        let connector: TlsConnector = Arc::new(config).into();
        connector
            .connect("www.rust-lang.org", tcp_stream)
            .await
            .context("Unable to establish TLS Connection")?
            .into()
    } else {
        tcp_stream.into()
    };
    write(&mut stream, format!("GET {} HTTP/1.0\n", PATH).as_bytes()).await?;
    write(&mut stream, format!("Host: {}\n", SERVER).as_bytes()).await?;
    write(
        &mut stream,
        "User-Agent: http_endless_header\n".as_bytes(),
    )
    .await?;
    write(&mut stream, b"Attack: ").await?;
    loop {
        if write(
            &mut stream,
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        )
        .await
        .is_err()
        {
            println!("Unable to send data. This might me the indicate of the server stopping the application from sending too much data");
            break;
        }
    }
    let mut buffer = String::with_capacity(4);
    let _bytes = stream.read_to_string(&mut buffer).await?;
    println!("{}", buffer);
    Ok(())
}

async fn write(stream: &mut MaybeHttpsStream, data: &[u8]) -> Result<()> {
    stream
        .write_all(data)
        .await
        .context("Unable to write Data to stream")
}
