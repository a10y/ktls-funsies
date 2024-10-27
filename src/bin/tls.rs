use std::{
    io::{stdout, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use rustls::{pki_types::ServerName, RootCertStore};

pub fn main() {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    };

    // Construct a new client connection.
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let server_name = ServerName::try_from("www.rust-lang.org").unwrap();

    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();

    let mut tcp = TcpStream::connect("www.rust-lang.org:443").unwrap();
    tcp.set_nodelay(true).unwrap();

    let mut tls = rustls::Stream::new(&mut conn, &mut tcp);

    // Write the GET request.
    let request = concat!(
        "GET / HTTP/1.1\r\n",
        "Host: www.rust-lang.org\r\n",
        "Connection: close\r\n",
        "Accept: */*\r\n",
        "User-Agent: curl/7.68.0\r\n",
        "\r\n",
    )
    .as_bytes();

    tls.write_all(request).unwrap();

    // Receiver response body until '\r\n\r\n'.
    let mut buf = Vec::with_capacity(16 * 1_024);
    tls.read_to_end(&mut buf).unwrap();
    stdout().write_all(&buf).unwrap();
}
