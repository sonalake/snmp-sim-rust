use crate::domain::ManagedDevice;
use crate::snmp::codec::snmp_codec::{GenericSnmpMessage, SnmpCodec};
use crate::udp_server::udp_server_error::UdpServerError;
use actix::prelude::*;
use actix::{Actor, Context, StreamHandler};
use futures::stream::SplitSink;
use futures::stream::StreamExt;
use std::net::SocketAddr;
use std::net::UdpSocket as StdUdpSocket;
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio_util::udp::UdpFramed;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) type UdpSinkItem = (GenericSnmpMessage, SocketAddr);

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) type UdpSplitSink = SplitSink<UdpFramed<SnmpCodec>, UdpSinkItem>;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) type UdpServerHandler = dyn Fn(GenericSnmpMessage, &ManagedDevice, SocketAddr, &mut UdpSplitSink);

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpStreamHandler {
    request_handler: &'static UdpServerHandler,
    device: ManagedDevice,
    sink: Option<UdpSplitSink>,
}

#[derive(Message)]
#[rtype(result = "()")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpMessage(pub Result<(GenericSnmpMessage, SocketAddr), String>);

impl UdpStreamHandler {
    pub fn new(request_handler: &'static UdpServerHandler, device: ManagedDevice) -> Result<Self, UdpServerError> {
        let actor = UdpStreamHandler {
            request_handler,
            device,
            sink: None,
        };
        Ok(actor)
    }
}

impl Actor for UdpStreamHandler {
    type Context = Context<Self>;

    #[tracing::instrument(level = "info", name = "UdpStreamHandler::started", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        let binding_address = format!("{}:{}", self.device.snmp_host, self.device.snmp_port);
        tracing::debug!("Bind a UDP listener to address: {}", binding_address);

        let std_socket = StdUdpSocket::bind(binding_address).unwrap();
        // socket must be set to unblocking mode to avoid thread hang
        std_socket.set_nonblocking(true).unwrap();
        let socket = TokioUdpSocket::from_std(std_socket).unwrap();
        let (sink, stream) = UdpFramed::new(socket, SnmpCodec::default()).split();
        self.sink = Some(sink);
        Self::add_stream(stream.map(|a| UdpMessage(a.map_err(|e| e.to_string()))), ctx);
    }
}

impl actix::Supervised for UdpStreamHandler {
    #[tracing::instrument(level = "info", name = "UdpStreamHandler::restarting", skip(self, _ctx))]
    fn restarting(&mut self, _ctx: &mut Context<Self>) {}
}

impl StreamHandler<UdpMessage> for UdpStreamHandler {
    #[tracing::instrument(level = "info", name = "UdpStreamHandler::UdpMessage", skip(self, data, _ctx))]
    fn handle(&mut self, data: UdpMessage, _ctx: &mut Self::Context) {
        match data.0 {
            Ok((message, peer)) => {
                // handle the SNMP request by calling the generic snmp message handler
                (self.request_handler)(message, &self.device, peer, self.sink.as_mut().unwrap());
            }

            Err(error) => {
                // we cannot do more than just log the error
                tracing::error!("Failed to read the message: {error}");
            }
        }
    }
}

#[derive(Clone, Debug, Message)]
#[rtype("()")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct StopActor;

impl Handler<StopActor> for UdpStreamHandler {
    type Result = ();

    #[tracing::instrument(level = "info", name = "UdpStreamHandler::StopActor", skip(self, ctx))]
    fn handle(&mut self, _: StopActor, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop()
    }
}
