use crate::domain::ManagedDevice;
use crate::domain::SnmpProtocolVersion;
use crate::domain::{handle_get_next_request, handle_get_request};
use crate::snmp::handlers::snmp_generic_handler::GenericHandlerError;
use crate::snmp::handlers::snmp_generic_handler::RequestContext;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;

use actix_async::address::Addr;
use snmp_data_parser::parser::snmp_data::component::SnmpData;
use std::net::SocketAddr;

#[tracing::instrument(level = "debug", name = "handle_snmp_message_v2", skip(snmp_data))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn handle_snmp_message_v2(
    v2_request: rasn_snmp::v2c::Message<rasn_snmp::v2::Pdus>,
    device: ManagedDevice,
    peer: SocketAddr,
    stream_handler_actor: Addr<UdpStreamHandler>,
    snmp_data: SnmpData,
) -> Result<(), GenericHandlerError> {
    match v2_request.data {
        rasn_snmp::v2::Pdus::GetRequest(snmp_get_request) => {
            handle_get_request(
                snmp_get_request.try_into()?,
                RequestContext::new(
                    device,
                    peer,
                    stream_handler_actor,
                    SnmpProtocolVersion::SNMPV2C(
                        std::str::from_utf8(&v2_request.community)
                            .unwrap_or("public")
                            .to_string(),
                    ),
                    snmp_data,
                ),
            )
            .await?;
        }
        rasn_snmp::v2::Pdus::GetNextRequest(get_next_request) => {
            handle_get_next_request(
                get_next_request.try_into()?,
                RequestContext::new(
                    device,
                    peer,
                    stream_handler_actor,
                    SnmpProtocolVersion::SNMPV2C(
                        std::str::from_utf8(&v2_request.community)
                            .unwrap_or("public")
                            .to_string(),
                    ),
                    snmp_data,
                ),
            )
            .await?;
        }
        rasn_snmp::v2::Pdus::Response(_) => { /* not handled by Agent */ }
        rasn_snmp::v2::Pdus::SetRequest(_) => {}
        rasn_snmp::v2::Pdus::GetBulkRequest(_bulk_request) => {}
        rasn_snmp::v2::Pdus::InformRequest(_) => {}
        rasn_snmp::v2::Pdus::Trap(_trap_request) => {}
        rasn_snmp::v2::Pdus::Report(_report) => {}
    }

    Ok(())
}
