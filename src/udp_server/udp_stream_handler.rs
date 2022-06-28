use crate::domain::AgentContext;
use crate::domain::ManagedDevice;
use crate::snmp::codec::generic_snmp_message::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;
use crate::udp_server::udp_server_error::UdpServerError;
use snmp_data_parser::parser::snmp_data::component::SnmpData;
use snmp_data_parser::parser::snmp_data::VeraxModifierExtractor;
use snmp_data_parser::SnmpDataParser;

use actix_async::address::Addr;
use actix_async::prelude::*;
use futures::future::{BoxFuture, Future};
use futures::stream::{SplitSink, StreamExt};
use futures::SinkExt;
use futures_util::FutureExt;
use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::net::UdpSocket as StdUdpSocket;
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio_util::udp::UdpFramed;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) type UdpSinkItem = (GenericSnmpMessage, SocketAddr);

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) type UdpSplitSink = SplitSink<UdpFramed<SnmpCodec>, UdpSinkItem>;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) type UdpServerHandler =
    dyn Fn(GenericSnmpMessage, ManagedDevice, SocketAddr, Addr<UdpStreamHandler>, SnmpData) -> BoxFuture<'static, ()>;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpStreamHandler {
    init_result: Option<Result<(), UdpServerError>>,
    request_handler: Box<UdpServerHandler>,
    device: ManagedDevice,
    sink: RefCell<Option<UdpSplitSink>>,
    snmp_data: Option<SnmpData>,
}
actor!(UdpStreamHandler);

impl fmt::Debug for UdpStreamHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "device={:?}", self.device)
    }
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpMessage(pub Result<(GenericSnmpMessage, SocketAddr), String>);
message!(UdpMessage, ());

impl UdpStreamHandler {
    pub async fn new<F, Fut>(request_handler: F, device: ManagedDevice) -> Result<Addr<Self>, UdpServerError>
    where
        F: Fn(GenericSnmpMessage, ManagedDevice, SocketAddr, Addr<UdpStreamHandler>, SnmpData) -> Fut + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let mut actor = UdpStreamHandler {
            init_result: None,
            request_handler: Box::new(move |message, device, peer, stream_handler_actor, snmp_data| {
                Box::pin(request_handler(message, device, peer, stream_handler_actor, snmp_data))
            }),
            device,
            sink: RefCell::new(None),
            snmp_data: None,
        };
        let actor_addr = UdpStreamHandler::create_async(move |ctx| {
            actor.init_result = Some(actor.bind_stream(ctx));
            async move {
                if let Some(Ok(_)) = actor.init_result {
                    actor.init_result = Some(actor.read_device_snmp_data().await);
                }
                actor
            }
        });
        let res = actor_addr
            .run(|act, _ctx| act.get_result().boxed_local())
            .await
            .map_err(|error| UdpServerError::StartFailed(error.to_string()))?;
        match res {
            Ok(_) => Ok(actor_addr),
            Err(err) => {
                actor_addr.stop(false);
                Err(err)
            }
        }
    }

    async fn get_result(&self) -> Result<(), UdpServerError> {
        self.init_result.clone().unwrap()
    }

    async fn read_device_snmp_data(&mut self) -> Result<(), UdpServerError> {
        let file_name = match &self.device.agent {
            crate::domain::ManagedDeviceAgent::Agent(agent) => agent.snmp_data_url.as_str(),
            crate::domain::ManagedDeviceAgent::Id(_agent_id) => todo!("retrieve agent by id from database"),
        };

        self.snmp_data = Some(read_snmp_data_file(file_name).await?);
        Ok(())
    }

    #[tracing::instrument(level = "info", name = "UdpStreamHandler::bind_stream", skip(self, ctx))]
    fn bind_stream(&self, ctx: Context<'_, Self>) -> Result<(), UdpServerError> {
        let binding_address = format!("{}:{}", self.device.snmp_host, self.device.snmp_port);
        tracing::debug!("Bind a UDP listener to address: {}", binding_address);

        let std_socket =
            StdUdpSocket::bind(binding_address).map_err(|error| UdpServerError::StartFailed(error.to_string()))?;

        // socket must be set to unblocking mode to avoid thread hang
        std_socket
            .set_nonblocking(true)
            .map_err(|error| UdpServerError::StartFailed(error.to_string()))?;
        let socket =
            TokioUdpSocket::from_std(std_socket).map_err(|error| UdpServerError::StartFailed(error.to_string()))?;

        // create a UdpFramed object using the SnmpCodec to work with the encoded/decoded frames directly, instead of raw UDP data
        let (sink, stream) = UdpFramed::new(socket, SnmpCodec::default()).split();
        let mut self_sink_mut = self.sink.borrow_mut();
        *self_sink_mut = Some(sink);

        // Add the stream to the actor's context.
        // Stream item will be treated as a concurrent message and the actor's handle will be called.
        ctx.add_stream(stream.map(|a| UdpMessage(a.map_err(|e| e.to_string()))));

        Ok(())
    }
}

async fn read_snmp_data_file(file_name: &str) -> Result<SnmpData, UdpServerError> {
    let input = BufReader::new(File::open(file_name).map_err(|error| UdpServerError::StartFailed(error.to_string()))?);
    let mut reader = SnmpDataParser::new(input, VeraxModifierExtractor {});

    match reader.next() {
        Some(data) => data.map_err(|error| UdpServerError::StartFailed(error.to_string())),
        _ => Ok(SnmpData::new()),
    }
}

#[actix_async::handler]
impl Handler<UdpMessage> for UdpStreamHandler {
    #[tracing::instrument(level = "info", name = "UdpStreamHandler::UdpMessage", skip(self, data, ctx))]
    async fn handle(&self, data: UdpMessage, ctx: Context<'_, Self>) {
        match data.0 {
            Ok((message, peer)) => {
                // handle the SNMP request by calling the generic snmp message handler
                (self.request_handler)(
                    message,
                    self.device.clone(),
                    peer,
                    ctx.address().unwrap(),
                    self.snmp_data.clone().unwrap(),
                )
                .await;
            }

            Err(error) => {
                // we cannot do more than just log the error
                tracing::error!("Failed to read the message: {error}");
            }
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct StopActor;
message!(StopActor, ());

#[actix_async::handler]
impl Handler<StopActor> for UdpStreamHandler {
    #[tracing::instrument(level = "info", name = "UdpStreamHandler::StopActor", skip(self, ctx))]
    async fn handle(&self, _: StopActor, ctx: Context<'_, Self>) -> () {
        ctx.stop()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct SendData {
    pub message: GenericSnmpMessage,
    pub peer: SocketAddr,
}
message!(SendData, ());

#[actix_async::handler]
impl Handler<SendData> for UdpStreamHandler {
    #[tracing::instrument(level = "info", name = "UdpStreamHandler::handle::SendData", skip(self, _ctx))]
    async fn handle(&self, data: SendData, _ctx: Context<'_, Self>) {
        let mut sink = self.sink.borrow_mut();
        // TODO, OPTIMIZE: change send to feed and wake up and flush it in 50ms
        let _ = sink.as_mut().unwrap().send((data.message, data.peer)).await;
    }
}

#[tracing::instrument(level = "info", name = "send_data")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) fn send_data(message: GenericSnmpMessage, request_context: &AgentContext) {
    request_context.stream_handler_actor.do_send(SendData {
        message,
        peer: request_context.peer,
    });
}
