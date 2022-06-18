use num_traits::ToPrimitive;
use rasn_snmp::v3::Message as SnmpV3Message;
use rasn_snmp::{v1::Message as SnmpV1Message, v1::Pdus as SnmpV1Pdus};
use rasn_snmp::{v2::Pdus as SnmpV2Pdus, v2c::Message as SnmpV2CMessage};

#[derive(Debug)]
pub enum GenericSnmpMessage {
    V1Message(SnmpV1Message<SnmpV1Pdus>),
    V2Message(SnmpV2CMessage<SnmpV2Pdus>),
    V3Message(Box<SnmpV3Message>), /* Large variant size differnce => use boxing to prevent the memory layout
                                    * penalization of that enum */
}

pub trait Id {
    fn id(&self) -> i32;
}

impl Id for GenericSnmpMessage {
    fn id(&self) -> i32 {
        match self {
            GenericSnmpMessage::V1Message(msg) => msg.id(),
            GenericSnmpMessage::V2Message(msg) => msg.id(),
            GenericSnmpMessage::V3Message(msg) => msg.id(),
        }
    }
}

impl Id for SnmpV1Message<SnmpV1Pdus> {
    fn id(&self) -> i32 {
        match &self.data {
            rasn_snmp::v1::Pdus::GetRequest(get) => get.0.request_id.to_i32().unwrap(),
            rasn_snmp::v1::Pdus::GetNextRequest(getnext) => getnext.0.request_id.to_i32().unwrap(),
            rasn_snmp::v1::Pdus::GetResponse(response) => response.0.request_id.to_i32().unwrap(),
            rasn_snmp::v1::Pdus::SetRequest(set) => set.0.request_id.to_i32().unwrap(),
            rasn_snmp::v1::Pdus::Trap(_trap) => unreachable!("SNMP v1 Trap message has no request_id"),
        }
    }
}

impl Id for SnmpV2CMessage<SnmpV2Pdus> {
    fn id(&self) -> i32 {
        match &self.data {
            rasn_snmp::v2::Pdus::GetRequest(get) => get.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::GetNextRequest(getnext) => getnext.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::Response(response) => response.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::SetRequest(set) => set.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::GetBulkRequest(getbulk) => getbulk.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::InformRequest(inform) => inform.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::Trap(trap) => trap.0.request_id.to_i32().unwrap(),
            rasn_snmp::v2::Pdus::Report(report) => report.0.request_id.to_i32().unwrap(),
        }
    }
}

impl Id for SnmpV3Message {
    fn id(&self) -> i32 {
        self.global_data.message_id.to_i32().unwrap()
    }
}
