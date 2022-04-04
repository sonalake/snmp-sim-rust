use crate::snmp::codec::snmp_codec::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;

use futures::stream::SplitSink;
use std::net::SocketAddr;
use tokio_util::udp::UdpFramed;

use super::snmp_v1_handler::*;
use super::snmp_v2_handler::*;
use super::snmp_v3_handler::*;

#[tracing::instrument(name = "handle_generic_snmp_message", level = "debug", skip(generic_request, sink))]
pub fn handle_generic_snmp_message(
    generic_request: GenericSnmpMessage,
    peer: SocketAddr,
    sink: &mut SplitSink<UdpFramed<SnmpCodec>, (GenericSnmpMessage, SocketAddr)>,
) {
    futures::executor::block_on(async {
        // Handle the generic_request
        if let Err(error) = match generic_request {
            GenericSnmpMessage::V1Message(message) => handle_snmp_message_v1(message, peer, sink).await,
            GenericSnmpMessage::V2Message(message) => handle_snmp_message_v2(message, peer, sink).await,
            GenericSnmpMessage::V3Message(message) => handle_snmp_message_v3(*message, peer, sink).await,
        } {
            tracing::error!("Request handler failed: {error}");
        }
    });
}
