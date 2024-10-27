use std::{
    io::{stdout, Write},
    sync::Arc,
};

use ktls::CorkStream;
use rustls::{client::Resumption, pki_types::ServerName, RootCertStore};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::TlsConnector;

#[tokio::main]
pub async fn main() {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    };

    // Construct a new client connection.
    let mut config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    config.enable_secret_extraction = true;
    config.resumption = Resumption::disabled();

    let tls_connector = TlsConnector::from(Arc::new(config));

    // Create a TCP connection to www.rust-lang.org
    let stream = TcpStream::connect("www.google.com:443").await.unwrap();
    // Wrap a CorkStream around it so we can use ktls
    let stream = CorkStream::new(stream);

    // connect
    let stream = tls_connector
        .connect("www.google.com".try_into().unwrap(), stream)
        .await
        .unwrap();

    // I suppose the handshake is done for is on top of the CorkStream.
    let mut tls = ktls::config_ktls_client(stream).await.unwrap();

    // Ok, now we can do simple reads/writes on the connection
    let request = concat!(
        "GET / HTTP/1.1\r\n",
        "Host: www.google.com\r\n",
        "Connection: close\r\n",
        "Accept: */*\r\n",
        "User-Agent: curl/7.68.0\r\n",
        "\r\n",
    )
    .as_bytes();

    tls.write_all(request).await.unwrap();

    // Receiver response body until '\r\n\r\n'.
    let mut buf = Vec::with_capacity(16 * 1_024);
    tls.read_to_end(&mut buf).await.unwrap();
    stdout().write_all(&buf).unwrap();
}
