mod snmp_v1;
mod snmp_v2;

use rasn::prelude::ObjectIdentifier;
use snmp_sim::domain::entity::{GetNextRequest, GetRequest};
use snmp_sim::domain::SnmpProtocolVersion;
use snmp_sim::snmp::codec::GenericSnmpMessage;
use static_init::dynamic;

pub fn get_request_v1(request_id: i32, community: &str, objects: Vec<ObjectIdentifier>) -> GenericSnmpMessage {
    get_request(request_id, &SnmpProtocolVersion::SNMPV1(community.to_string()), objects)
}

pub fn get_request_v2(request_id: i32, community: &str, objects: Vec<ObjectIdentifier>) -> GenericSnmpMessage {
    get_request(
        request_id,
        &SnmpProtocolVersion::SNMPV2C(community.to_string()),
        objects,
    )
}

fn get_request(request_id: i32, protocol: &SnmpProtocolVersion, objects: Vec<ObjectIdentifier>) -> GenericSnmpMessage {
    let response = GetRequest { request_id, objects };
    (protocol, response).into()
}

#[allow(dead_code)]
pub fn get_next_request_v1(request_id: i32, community: &str, objects: Vec<ObjectIdentifier>) -> GenericSnmpMessage {
    get_next_request(request_id, &SnmpProtocolVersion::SNMPV1(community.to_string()), objects)
}

#[allow(dead_code)]
pub fn get_next_request_v2(request_id: i32, community: &str, objects: Vec<ObjectIdentifier>) -> GenericSnmpMessage {
    get_next_request(
        request_id,
        &SnmpProtocolVersion::SNMPV2C(community.to_string()),
        objects,
    )
}

#[allow(dead_code)]
pub fn get_next_request(
    request_id: i32,
    protocol: &SnmpProtocolVersion,
    objects: Vec<ObjectIdentifier>,
) -> GenericSnmpMessage {
    let response = GetNextRequest { request_id, objects };
    (protocol, response).into()
}

#[dynamic]
pub static mut DEVICE_PORT: DevicePort = DevicePort::new(30160);

pub struct DevicePort {
    value: u16,
}

impl DevicePort {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn get_next_value(&mut self) -> u16 {
        self.value += 1;
        self.value
    }
}
