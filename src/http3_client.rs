use quiche::{h3, Config};
use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

pub struct Http3Client {
    conn: quiche::Connection,
    h3_conn: h3::Connection,
}

impl Default for Http3Client {
    fn default() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let mut config = Config::new(quiche::PROTOCOL_VERSION).expect("Failed to create config");
        config.set_application_protos(&[b"example-proto"]).unwrap();
        config.verify_peer(false);

        let scid = quiche::ConnectionId::from_ref(&[0xba; 16]);

        let local_addr = socket.local_addr().expect("Failed to get local address");
        let peer_addr =
            SocketAddr::from_str("profiling.lemon.industries:443").expect("Invalid peer address");

        let mut conn = quiche::connect(None, &scid, local_addr, peer_addr, &mut config)
            .expect("Failed to create connection");
        let h3_conn = h3::Connection::with_transport(
            &mut conn,
            &h3::Config::new().expect("Failed to create h3 config"),
        )
        .expect("Failed to create h3 connection");

        Self { conn, h3_conn }
    }
}

impl Http3Client {
    pub fn request(&mut self) -> Result<(), quiche::h3::Error> {
        let req = vec![
            h3::Header::new(b":method", b"GET"),
            h3::Header::new(b":scheme", b"https"),
            h3::Header::new(b":authority", b"profiling.lemon.industries"),
            h3::Header::new(b":path", b"/"),
        ];

        self.h3_conn.send_request(&mut self.conn, &req, true)?;
        Ok(())
    }
}
