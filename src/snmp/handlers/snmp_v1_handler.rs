use crate::domain::to_string_default;
use crate::domain::ManagedDevice;
use crate::domain::SnmpProtocolVersion;
use crate::domain::{handle_get_next_request, handle_get_request};
use crate::snmp::handlers::snmp_generic_handler::GenericHandlerError;
use crate::snmp::handlers::snmp_generic_handler::RequestContext;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;

use actix_async::address::Addr;
use snmp_data_parser::parser::snmp_data::component::SnmpData;
use std::net::SocketAddr;

#[tracing::instrument(level = "debug", name = "handle_snmp_message_v1", skip(snmp_data))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn handle_snmp_message_v1(
    v1_request: rasn_snmp::v1::Message<rasn_snmp::v1::Pdus>,
    device: ManagedDevice,
    peer: SocketAddr,
    stream_handler_actor: Addr<UdpStreamHandler>,
    snmp_data: SnmpData,
) -> Result<(), GenericHandlerError> {
    match v1_request.data {
        rasn_snmp::v1::Pdus::GetRequest(snmp_get_request) => {
            handle_get_request(
                snmp_get_request.try_into()?,
                RequestContext::new(
                    device,
                    peer,
                    stream_handler_actor,
                    SnmpProtocolVersion::SNMPV1(to_string_default(&v1_request.community, "public")),
                    snmp_data,
                ),
            )
            .await?;
        }
        rasn_snmp::v1::Pdus::GetNextRequest(snmp_get_next_request) => {
            handle_get_next_request(
                snmp_get_next_request.try_into()?,
                RequestContext::new(
                    device,
                    peer,
                    stream_handler_actor,
                    SnmpProtocolVersion::SNMPV1(to_string_default(&v1_request.community, "public")),
                    snmp_data,
                ),
            )
            .await?;
        }
        rasn_snmp::v1::Pdus::GetResponse(_) => { /* not handled by Agent */ }
        rasn_snmp::v1::Pdus::SetRequest(_) => {}
        rasn_snmp::v1::Pdus::Trap(_trap_request) => {}
    };

    Ok(())
}
