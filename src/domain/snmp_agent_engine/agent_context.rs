use crate::domain::ManagedDevice;
use crate::domain::SnmpProtocolVersion;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;

use actix_async::address::Addr;
use snmp_data_parser::parser::snmp_data::component::SnmpData;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct AgentContext {
    pub device: ManagedDevice,
    pub peer: SocketAddr,
    pub stream_handler_actor: Addr<UdpStreamHandler>,
    pub version: SnmpProtocolVersion,
    pub snmp_data: Arc<SnmpData>,
}

impl AgentContext {
    pub fn new(
        device: ManagedDevice,
        peer: SocketAddr,
        stream_handler_actor: Addr<UdpStreamHandler>,
        version: SnmpProtocolVersion,
        snmp_data: Arc<SnmpData>,
    ) -> Self {
        AgentContext {
            device,
            peer,
            stream_handler_actor,
            version,
            snmp_data,
        }
    }
}

impl fmt::Debug for AgentContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "device={:?}, peer={:?}, version={:?}",
            self.device, self.peer, self.version
        )
    }
}
