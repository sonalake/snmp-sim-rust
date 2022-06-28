use crate::domain::ManagedDevice;
use crate::domain::SnmpAgentCommandResponderError;
use crate::domain::ValidationError;
use crate::snmp::codec::generic_snmp_message::GenericSnmpMessage;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;

use actix_async::address::Addr;
use shared_common::error_chain_fmt;
use snmp_data_parser::parser::snmp_data::component::SnmpData;
use std::convert::Infallible;
use std::net::SocketAddr;

use super::snmp_v1_handler::*;
use super::snmp_v2_handler::*;
use super::snmp_v3_handler::*;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(thiserror::Error)]
pub(crate) enum GenericHandlerError {
    #[error(transparent)]
    SnmpAgent(#[from] SnmpAgentCommandResponderError),

    #[error(transparent)]
    Validation(#[from] ValidationError),
}

impl std::fmt::Debug for GenericHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for GenericHandlerError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to GenericHandlerError")
    }
}

#[tracing::instrument(
    level = "debug",
    name = "generic_snmp_message_handler",
    skip(generic_request, stream_handler_actor, snmp_data)
)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn generic_snmp_message_handler(
    generic_request: GenericSnmpMessage,
    device: ManagedDevice,
    peer: SocketAddr,
    stream_handler_actor: Addr<UdpStreamHandler>,
    snmp_data: SnmpData,
) {
    // Handle the generic_request
    if let Err(error) = match generic_request {
        GenericSnmpMessage::V1Message(message) => {
            handle_snmp_message_v1(message, device, peer, stream_handler_actor, snmp_data).await
        }
        GenericSnmpMessage::V2Message(message) => {
            handle_snmp_message_v2(message, device, peer, stream_handler_actor, snmp_data).await
        }
        GenericSnmpMessage::V3Message(message) => {
            handle_snmp_message_v3(*message, device, peer, stream_handler_actor, snmp_data).await
        }
    } {
        tracing::error!("Request handler failed: {error}");
    }
}
