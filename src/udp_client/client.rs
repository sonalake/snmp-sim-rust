use crate::snmp::codec::generic_snmp_message::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;
use crate::udp_client::ClientError;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::net::SocketAddr;
use std::net::UdpSocket as StdUdpSocket;
use std::{net::ToSocketAddrs, time::Duration};
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio_util::udp::UdpFramed;

// Timeout in seconds.
const TIMEOUT: u64 = 3;

// Client to send and receive SNMP messages. Only supports IPv4.
pub struct Client {
    udp_framed: UdpFramed<SnmpCodec>,
    remote_addr: SocketAddr,
}

impl Client {
    // Constructs a new `Client` and connect it to the remote address using UDP as a transport protocol.
    pub fn new(remote_addr: SocketAddr) -> Result<Client, ClientError> {
        let timeout = Some(Duration::from_secs(TIMEOUT));
        let std_socket = StdUdpSocket::bind("0.0.0.0:0")?;
        std_socket.set_read_timeout(timeout)?;
        std_socket.set_write_timeout(timeout)?;
        std_socket.connect(remote_addr)?;
        let socket = TokioUdpSocket::from_std(std_socket)?;
        let udp_framed = UdpFramed::new(socket, SnmpCodec::default());

        Ok(Self {
            udp_framed,
            remote_addr: remote_addr.to_socket_addrs().unwrap().next().unwrap(),
        })
    }

    // Sends a request and returns the response on success.
    pub async fn send_request(&mut self, msg: GenericSnmpMessage) -> Result<GenericSnmpMessage, ClientError> {
        self.send_message(msg).await?;
        self.recv_message().await
    }

    pub async fn send_message(&mut self, msg: GenericSnmpMessage) -> Result<(), ClientError> {
        self.udp_framed
            .send((msg, self.remote_addr))
            .await
            .map_err(ClientError::from)
    }

    pub async fn recv_message(&mut self) -> Result<GenericSnmpMessage, ClientError> {
        match self.udp_framed.next().await {
            Some(result) => result
                .map(|(message, _)| message)
                .map_err(ClientError::from),
            None => Err(ClientError::StreamClosed),
        }
    }
}
