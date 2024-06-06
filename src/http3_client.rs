use quiche::{h3, Config};
use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

pub struct Http3Client {
    socket: UdpSocket,
    config: Config,
    conn: quiche::Connection,
}

impl Http3Client {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let mut config = Config::new(quiche::PROTOCOL_VERSION).unwrap();
        config.verify_peer(false);

        let scid = quiche::ConnectionId::from_ref(&[0xba; 16]);

        let local_addr = SocketAddr::from_str("0.0.0.0:0").unwrap();
        let peer_addr = SocketAddr::from_str("0.0.0.0:0").unwrap();

        let conn = quiche::connect(None, &scid, local_addr, peer_addr, &mut config).unwrap();

        Self {
            socket,
            config,
            conn,
        }
    }

    pub fn request(&mut self) -> Result<(), quiche::h3::Error> {
        let mut h3_conn =
            h3::Connection::with_transport(&mut self.conn, &h3::Config::new().unwrap()).unwrap();

        let req = vec![
            h3::Header::new(b":method", b"GET"),
            h3::Header::new(b":scheme", b"https"),
            h3::Header::new(b":authority", b"profiling.lemon.industries"),
            h3::Header::new(b":path", b"/"),
        ];

        h3_conn.send_request(&mut self.conn, &req, true)?;
        Ok(())
    }
}
