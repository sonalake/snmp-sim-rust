use crate::snmp::codec::snmp_codec::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;
use crate::snmp::codec::snmp_codec_error::CodecError;

use futures::stream::SplitSink;
use std::net::SocketAddr;
use tokio_util::udp::UdpFramed;

// use futures::prelude::*;
// use rasn::prelude::Integer;
// use rasn_smi::v2::ObjectSyntax;
// use rasn_smi::v2::SimpleSyntax;
// use rasn_snmp::v1::GetResponse;
// use rasn_snmp::v1::Pdus::GetRequest;

#[tracing::instrument(name = "handle_snmp_message_v3", level = "info", skip(_sink, _peer))]
pub async fn handle_snmp_message_v3(
    _message: rasn_snmp::v3::Message,
    _peer: SocketAddr,
    _sink: &mut SplitSink<UdpFramed<SnmpCodec>, (GenericSnmpMessage, SocketAddr)>,
) -> Result<(), CodecError> {
    Ok(())
}
