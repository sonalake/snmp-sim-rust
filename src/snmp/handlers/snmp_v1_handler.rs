use crate::snmp::codec::snmp_codec::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;
use crate::snmp::codec::snmp_codec_error::CodecError;

use futures::prelude::*;
use futures::stream::SplitSink;
use rasn::prelude::Integer;
use rasn_smi::v1::ObjectSyntax;
use rasn_smi::v1::SimpleSyntax;
use rasn_snmp::v1::GetResponse;
use rasn_snmp::v1::Pdus::GetRequest;
use std::net::SocketAddr;
use tokio_util::udp::UdpFramed;

#[tracing::instrument(name = "handle_snmp_message_v1", level = "info", skip(sink, peer))]
pub async fn handle_snmp_message_v1(
    v1_request: rasn_snmp::v1::Message<rasn_snmp::v1::Pdus>,
    peer: SocketAddr,
    sink: &mut SplitSink<UdpFramed<SnmpCodec>, (GenericSnmpMessage, SocketAddr)>,
) -> Result<(), CodecError> {
    match v1_request.data {
        GetRequest(get_request_pdu) => {
            // TODO: intern implementation to be replaced
            // the following code repesents an example how to construct and send a SNMP get
            // response message
            let mut get_response_pdu = get_request_pdu.0.clone();
            get_response_pdu.variable_bindings[0].value =
                ObjectSyntax::Simple(SimpleSyntax::Number(Integer::from(0 as i32)));
            let response: rasn_snmp::v1::Message<rasn_snmp::v1::Pdus> = rasn_snmp::v1::Message {
                version: v1_request.version,
                community: v1_request.community,
                data: rasn_snmp::v1::Pdus::GetResponse(GetResponse(get_response_pdu)),
            };

            sink.send((GenericSnmpMessage::V1Message(response), peer))
                .await?
        }
        _ => {}
    };

    Ok(())
}
