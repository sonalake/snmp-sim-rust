use crate::domain::ManagedDevice;
use std::sync::Arc;
//use crate::domain::SnmpProtocolVersion;
//use crate::snmp::handlers::snmp_generic_handler::AgentContext;
use crate::snmp::handlers::snmp_generic_handler::GenericHandlerError;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;

use actix_async::address::Addr;
use snmp_data_parser::parser::snmp_data::component::SnmpData;
use std::net::SocketAddr;

// use futures::prelude::*;
// use rasn::prelude::Integer;
// use rasn_smi::v2::ObjectSyntax;
// use rasn_smi::v2::SimpleSyntax;
// use rasn_snmp::v1::GetResponse;
// use rasn_snmp::v1::Pdus::GetRequest;

#[tracing::instrument(level = "debug", name = "handle_snmp_message_v3", skip(_snmp_data))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn handle_snmp_message_v3(
    _message: rasn_snmp::v3::Message,
    _device: ManagedDevice,
    _peer: SocketAddr,
    _stream_handler_actor: Addr<UdpStreamHandler>,
    _snmp_data: Arc<SnmpData>,
) -> Result<(), GenericHandlerError> {
    Ok(())
}
