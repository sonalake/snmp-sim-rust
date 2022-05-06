use crate::snmp::codec::snmp_codec::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;
use crate::snmp::codec::snmp_codec_error::CodecError;

use futures::stream::SplitSink;
use std::net::SocketAddr;
use tokio_util::udp::UdpFramed;

use futures::prelude::*;
use rasn::prelude::Integer;
use rasn_smi::v1::ObjectSyntax;
use rasn_smi::v1::SimpleSyntax;
use rasn_snmp::v1::GetResponse;
use rasn_snmp::v2::Pdus::GetRequest;

#[tracing::instrument(level = "debug", name = "handle_snmp_message_v2", skip(sink, peer))]
pub async fn handle_snmp_message_v2(
    v2_request: rasn_snmp::v2c::Message<rasn_snmp::v2::Pdus>,
    peer: SocketAddr,
    sink: &mut SplitSink<UdpFramed<SnmpCodec>, (GenericSnmpMessage, SocketAddr)>,
) -> Result<(), CodecError> {
    if let GetRequest(get_request_pdu) = v2_request.data {
        // TODO: intern implementation to be replaced
        // the following code repesents an example how to construct and send a SNMP get
        // response message
        let mut get_response_pdu = get_request_pdu.0.clone();
        get_response_pdu.variable_bindings[0].value = ObjectSyntax::Simple(SimpleSyntax::Number(Integer::from(0_i32)));
        let response: rasn_snmp::v2c::Message<rasn_snmp::v2::Pdus> = rasn_snmp::v2c::Message {
            version: v2_request.version,
            community: v2_request.community,
            data: rasn_snmp::v2::Pdus::Response(GetResponse(get_response_pdu)),
        };

        sink.send((GenericSnmpMessage::V2Message(response), peer))
            .await?
    }

    Ok(())
}
