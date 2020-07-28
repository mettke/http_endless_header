use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use futures::{
    io::{self, AsyncRead, AsyncWrite},
    task,
};
use std::pin::Pin;

#[allow(clippy::large_enum_variant, variant_size_differences)]
#[derive(Debug)]
/// Abstracts whether the internal Stream is encrypted or not
pub enum MaybeHttpsStream {
    /// Inner stream is not encrypted
    Http(TcpStream),
    /// Inner stream is encrypted
    Https(TlsStream<TcpStream>),
}

impl From<TcpStream> for MaybeHttpsStream {
    #[inline]
    fn from(stream: TcpStream) -> Self {
        Self::Http(stream)
    }
}

impl From<TlsStream<TcpStream>> for MaybeHttpsStream {
    #[inline]
    fn from(stream: TlsStream<TcpStream>) -> Self {
        Self::Https(stream)
    }
}

impl AsyncRead for MaybeHttpsStream {
    #[inline]
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
    #[inline]
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

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<io::Result<()>> {
        match *self {
            Self::Http(ref mut s) => Pin::new(s).poll_flush(cx),
            Self::Https(ref mut s) => Pin::new(s).poll_flush(cx),
        }
    }

    #[inline]
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
