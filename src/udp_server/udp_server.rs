use actix::prelude::*;
use actix::{Actor, Context, StreamHandler};
use futures::prelude::*;
use futures::stream::SplitSink;
use std::fmt::Display;
use std::net::SocketAddr;
use std::net::UdpSocket as StdUdpSocket;
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::udp::UdpFramed;

pub struct UdpServer<Codec, Message, Closure> {
    sink: Option<SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>>,
    host: String,
    port: u16,
    request_handler: Closure,
}

impl<Codec, Message, Closure> UdpServer<Codec, Message, Closure>
where
    Codec: Default + Decoder<Item = Message> + Encoder<Message>,
    <Codec as Decoder>::Error: Display,
    Closure: FnOnce(Message, SocketAddr, &mut SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>),
{
    pub fn new(host: String, port: u16, request_handler: Closure) -> Self {
        UdpServer {
            sink: None,
            host,
            port,
            request_handler,
        }
    }
}

// implement the Supervised trait to create a new execution context and restart
// the actor’s lifecycle in case of actor failure
impl<Codec: 'static, Message: 'static, Closure: 'static> actix::Supervised for UdpServer<Codec, Message, Closure>
where
    Codec: Default + Decoder<Item = Message> + Encoder<Message>,
    <Codec as Decoder>::Error: Display,
    Closure: Fn(Message, SocketAddr, &mut SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>) + std::marker::Unpin,
{
    #[tracing::instrument(name = "UdpServer::restarting", level = "debug", skip(self, _ctx))]
    fn restarting(&mut self, _ctx: &mut Context<Self>) {}
}

#[derive(Message)]
#[rtype(result = "()")]
struct UdpDataMsg<Message>(pub Result<(Message, SocketAddr), String>);

impl<Codec: 'static, Message: 'static, Closure: 'static> Actor for UdpServer<Codec, Message, Closure>
where
    Codec: Default + Decoder<Item = Message> + Encoder<Message>,
    <Codec as Decoder>::Error: Display,
    Closure: Fn(Message, SocketAddr, &mut SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>) + std::marker::Unpin,
{
    type Context = Context<Self>;

    #[tracing::instrument(name = "UdpServer::started", level = "debug", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        let binding_address = format!("{}:{}", self.host, self.port);
        tracing::debug!("UdpServer binding address: {}", binding_address);

        let std_socket = StdUdpSocket::bind(binding_address).unwrap();
        let sock = TokioUdpSocket::from_std(std_socket).unwrap();

        // bind the socket to the protocol decoder
        let (sink, stream) = UdpFramed::new(sock, Codec::default()).split();

        // initialize an actor stream consumer of the decoded requests
        Self::add_stream(stream.map(|a| UdpDataMsg::<Message>(a.map_err(|e| e.to_string()))), ctx);

        self.sink = Some(sink);
    }
}

impl<Codec: 'static, Message: 'static, Closure: 'static> StreamHandler<UdpDataMsg<Message>>
    for UdpServer<Codec, Message, Closure>
where
    Codec: Default + Decoder<Item = Message> + Encoder<Message>,
    <Codec as Decoder>::Error: Display,
    Closure: Fn(Message, SocketAddr, &mut SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>) + std::marker::Unpin,
{
    #[tracing::instrument(name = "UdpServer::handle", level = "debug", skip(self, data, _ctx))]
    fn handle(&mut self, data: UdpDataMsg<Message>, _ctx: &mut Self::Context) {
        match data.0 {
            Ok((message, peer)) => {
                // Handle the request
                (self.request_handler)(message, peer, self.sink.as_mut().unwrap());
            }

            Err(error) => {
                tracing::error!("Error occured: {error}");
            }
        }
    }
}
