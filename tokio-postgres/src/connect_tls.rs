use crate::config::SslMode;
use crate::maybe_tls_stream::MaybeTlsStream;
use crate::tls::private::ForcePrivateApi;
use crate::tls::{ChannelBinding, TlsConnect};
use crate::Error;
use postgres_protocol::message::frontend;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use bytes::BytesMut;

pub async fn connect_tls<S, T>(
    mut stream: S,
    mode: SslMode,
    tls: T,
) -> Result<(MaybeTlsStream<S, T::Stream>, ChannelBinding), Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
    T: TlsConnect<S>,
{
    match mode {
        SslMode::Disable => return Ok((MaybeTlsStream::Raw(stream), ChannelBinding::none())),
        SslMode::Prefer if !tls.can_connect(ForcePrivateApi) => {
            return Ok((MaybeTlsStream::Raw(stream), ChannelBinding::none()))
        }
        SslMode::Prefer | SslMode::Require => {}
        SslMode::__NonExhaustive => unreachable!(),
    }

    let mut buf = BytesMut::new();
    frontend::ssl_request(&mut buf);
    stream.write_all(&buf).await.map_err(Error::io)?;

    let mut buf = [0];
    stream.read_exact(&mut buf).await.map_err(Error::io)?;

    if buf[0] != b'S' {
        if SslMode::Require == mode {
            return Err(Error::tls("server does not support TLS".into()));
        } else {
            return Ok((MaybeTlsStream::Raw(stream), ChannelBinding::none()));
        }
    }

    let (stream, channel_binding) = tls
        .connect(stream)
        .await
        .map_err(|e| Error::tls(e.into()))?;

    Ok((MaybeTlsStream::Tls(stream), channel_binding))
}
