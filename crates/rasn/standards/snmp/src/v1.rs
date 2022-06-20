//! Version 1 (RFC 1157)

use rasn::{
    types::{Integer, ObjectIdentifier, OctetString},
    AsnType, Decode, Encode,
};
use smi::v1::{NetworkAddress, ObjectName, ObjectSyntax, TimeTicks};

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Message<T> {
    pub version: Integer,
    pub community: OctetString,
    pub data: T,
}

impl<T> Message<T> {
    pub const VERSION_1: u64 = 0;
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum Pdus {
    GetRequest(GetRequest),
    GetNextRequest(GetNextRequest),
    GetResponse(GetResponse),
    SetRequest(SetRequest),
    Trap(Trap),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(0))]
#[rasn(delegate)]
pub struct GetRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(1))]
#[rasn(delegate)]
pub struct GetNextRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(2))]
#[rasn(delegate)]
pub struct GetResponse(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(3))]
#[rasn(delegate)]
pub struct SetRequest(pub Pdu);

pub type VarBindList = alloc::vec::Vec<VarBind>;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pdu {
    pub request_id: Integer,
    pub error_status: Integer,
    pub error_index: Integer,
    pub variable_bindings: VarBindList,
}

impl Pdu {
    pub const ERROR_STATUS_NO_ERROR: u64 = 0;
    pub const ERROR_STATUS_TOO_BIG: u64 = 1;
    pub const ERROR_STATUS_NO_SUCH_NAME: u64 = 2;
    pub const ERROR_STATUS_BAD_VALUE: u64 = 3;
    pub const ERROR_STATUS_READ_ONLY: u64 = 4;
    pub const ERROR_STATUS_GEN_ERR: u64 = 5;
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(context, 4))]
pub struct Trap {
    pub enterprise: ObjectIdentifier,
    pub agent_addr: NetworkAddress,
    pub generic_trap: Integer,
    pub specific_trap: Integer,
    pub time_stamp: TimeTicks,
    pub variable_bindings: VarBindList,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VarBind {
    pub name: ObjectName,
    pub value: ObjectSyntax,
}

#[cfg(test)]
mod tests {
    use crate::v1::{GetRequest, GetResponse, Message, Pdu, Trap, VarBind};
    use alloc::{string::String, string::ToString, vec, vec::Vec};
    use rasn::ber::de::DecoderOptions;
    use rasn::types::ObjectIdentifier;
    use rasn::Decode;
    use smi::v1::{Gauge, IpAddress, NetworkAddress, ObjectSyntax, SimpleSyntax, TimeTicks};

    fn oid_to_string(oid: impl AsRef<[u32]>) -> String {
        oid.as_ref()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(".")
    }

    fn string_to_oid(oid: &str) -> ObjectIdentifier {
        rasn::types::ObjectIdentifier::new(
            oid.trim_matches('.')
                .split('.')
                .map(|val| val.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .to_vec(),
        )
        .unwrap()
    }

    pub fn decode<T: Decode>(decoder: &mut rasn::ber::de::Decoder) -> Result<T, rasn::ber::de::Error> {
        T::decode(decoder)
    }

    #[test]
    fn get_request_var_bind_array() {
        let encode_msg = Message {
            version: 0.into(),
            community: "public".into(),
            data: GetRequest(Pdu {
                request_id: 1.into(),
                error_index: 0.into(),
                error_status: 0.into(),
                variable_bindings: vec![
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 1, 3].into()),
                        value: ObjectSyntax::Simple(SimpleSyntax::Empty),
                    },
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 4, 1, 11779, 1, 42, 2, 1, 7].into()),
                        value: ObjectSyntax::Simple(SimpleSyntax::Empty),
                    },
                ],
            }),
        };

        let encode_data = rasn::ber::encode(&encode_msg).unwrap();
        let decode_msg: Message<GetRequest> = rasn::ber::decode(&encode_data).unwrap();
        assert_eq!(encode_msg, decode_msg);
    }

    #[test]
    fn get_response_var_bind_array() {
        let encode_msg = Message {
            version: 0.into(),
            community: "public".into(),
            data: GetResponse(Pdu {
                request_id: 1.into(),
                error_index: 0.into(),
                error_status: 0.into(),
                variable_bindings: vec![
                    VarBind {
                        name: string_to_oid("1.3.6.1.2.1.1.1.0"),
                        value: ObjectSyntax::Simple(SimpleSyntax::String(
                            "Linux nmsworker-devel 2.6.18-164.el5 #1 SMP Thu Sep 3 03:28:30 EDT 2009 x86_64".into(),
                        )),
                    },
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 1, 2, 0].into()),
                        value: ObjectSyntax::Simple(SimpleSyntax::Object(string_to_oid("1.3.6.1.4.1.8072.3.2.10"))),
                    },
                    VarBind {
                        name: string_to_oid("1.3.6.1.2.1.1.6.0"),
                        value: ObjectSyntax::Simple(SimpleSyntax::String("Unknown (edit /etc/snmp/snmpd.conf)".into())),
                    },
                ],
            }),
        };

        let encoded_data = rasn::der::encode(&encode_msg).unwrap();
        let expected_data = [
            0x30, 0x81, 0xBF, 0x02, 0x01, 0x00, 0x04, 0x06, 0x70, 0x75, 0x62, 0x6C, 0x69, 0x63, 0xA2, 0x81, 0xB1, 0x02,
            0x01, 0x01, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x30, 0x81, 0xA5, 0x30, 0x5A, 0x06, 0x08, 0x2B, 0x06, 0x01,
            0x02, 0x01, 0x01, 0x01, 0x00, 0x04, 0x4E, 0x4C, 0x69, 0x6E, 0x75, 0x78, 0x20, 0x6E, 0x6D, 0x73, 0x77, 0x6F,
            0x72, 0x6B, 0x65, 0x72, 0x2D, 0x64, 0x65, 0x76, 0x65, 0x6C, 0x20, 0x32, 0x2E, 0x36, 0x2E, 0x31, 0x38, 0x2D,
            0x31, 0x36, 0x34, 0x2E, 0x65, 0x6C, 0x35, 0x20, 0x23, 0x31, 0x20, 0x53, 0x4D, 0x50, 0x20, 0x54, 0x68, 0x75,
            0x20, 0x53, 0x65, 0x70, 0x20, 0x33, 0x20, 0x30, 0x33, 0x3A, 0x32, 0x38, 0x3A, 0x33, 0x30, 0x20, 0x45, 0x44,
            0x54, 0x20, 0x32, 0x30, 0x30, 0x39, 0x20, 0x78, 0x38, 0x36, 0x5F, 0x36, 0x34, 0x30, 0x16, 0x06, 0x08, 0x2B,
            0x06, 0x01, 0x02, 0x01, 0x01, 0x02, 0x00, 0x06, 0x0A, 0x2B, 0x06, 0x01, 0x04, 0x01, 0xBF, 0x08, 0x03, 0x02,
            0x0A, 0x30, 0x2F, 0x06, 0x08, 0x2B, 0x06, 0x01, 0x02, 0x01, 0x01, 0x06, 0x00, 0x04, 0x23, 0x55, 0x6E, 0x6B,
            0x6E, 0x6F, 0x77, 0x6E, 0x20, 0x28, 0x65, 0x64, 0x69, 0x74, 0x20, 0x2F, 0x65, 0x74, 0x63, 0x2F, 0x73, 0x6E,
            0x6D, 0x70, 0x2F, 0x73, 0x6E, 0x6D, 0x70, 0x64, 0x2E, 0x63, 0x6F, 0x6E, 0x66, 0x29,
        ];
        assert_eq!(encoded_data, expected_data);
        let mut decoder = rasn::ber::de::Decoder::new(&encoded_data, DecoderOptions::der());
        let decode_msg: Message<GetResponse> = decode(&mut decoder).unwrap();
        assert_eq!(encode_msg, decode_msg);
    }

    #[test]
    fn trap() {
        #[rustfmt::skip]
        let decode_data = [
            // SEQUENCE -> Message
            0x30, 0x4f,
                // INTEGER -> Message::version
                0x02, 0x01,
                    0x00,
                // OCTET STRING -> Message::community
                0x04, 0x06,
                    // "public"
                    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63,
                // application constructed tag 4 -> Trap
                0xa4, 0x42,
                    // OID -> Trap::enterprise
                    0x06, 0x0c,
                        // 1.3.6.1.4.1.11779.1.42.3.7.8
                        0x2b, 0x06, 0x01, 0x04, 0x01, 0xDC, 0x03, 0x01,
                        0x2a, 0x03, 0x07, 0x08,
                    // OCTET STRING -> Trap::agent_addr
                    0x40, 0x04,
                        // NetworkAddress:Internet(IpAddress(10.11.12.13))
                        0x0a, 0x0b, 0x0c, 0x0d,
                    // INTEGER -> Trap::generic_trap
                    0x02, 0x01,
                        0x06,
                    // INTEGER -> Trap::specific_trap
                    0x02, 0x01,
                        0x02,
                    // application tag 3 -> TimeTicks
                    0x43, 0x02,
                        // 11_932
                        0x2e, 0x9c,
                    // SEQUENCE -> VarBindList
                    0x30, 0x22,
                        // SEQUENCE -> VarBind
                        0x30, 0x0d,
                            // OID -> VarBind::name
                            0x06, 0x07,
                                // 1.3.6.1.2.1.1.3
                                0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x03,
                            // application tag 3 -> TimeTicks
                            0x43, 0x02,
                                // 11_932
                                0x2e, 0x9c,
                        // SEQUENCE -> VarBind
                        0x30, 0x11,
                            // OID -> VarBind::name
                            0x06, 0x0c,
                                // 1.3.6.1.4.1.11779.1.42.2.1.7
                                0x2b, 0x06, 0x01, 0x04, 0x01, 0xDC, 0x03, 0x01,
                                0x2a, 0x02, 0x01, 0x07,
                            // application tag 2 -> Gauge
                            0x42, 0x01,
                                0x01,
        ];
        let decode_msg: Message<Trap> = rasn::ber::decode(&decode_data).unwrap();
        assert_eq!(decode_msg.version, 0.into());
        assert_eq!(decode_msg.community, "public".as_bytes());
        assert_eq!(
            oid_to_string(decode_msg.data.enterprise),
            "1.3.6.1.4.1.11779.1.42.3.7.8"
        );
        assert_eq!(
            decode_msg.data.agent_addr,
            NetworkAddress::Internet(IpAddress([10, 11, 12, 13][..].into()))
        );
        assert_eq!(decode_msg.data.generic_trap, 6.into());
        assert_eq!(decode_msg.data.specific_trap, 2.into());
        assert_eq!(decode_msg.data.time_stamp, TimeTicks(11_932));
        assert_eq!(decode_msg.data.variable_bindings.len(), 2);

        let encode_msg = Message {
            version: 0.into(),
            community: "public".into(),
            data: Trap {
                enterprise: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 4, 1, 11779, 1, 42, 3, 7, 8].into()),
                agent_addr: NetworkAddress::Internet(IpAddress([10, 11, 12, 13][..].into())),
                generic_trap: 6.into(),
                specific_trap: 2.into(),
                time_stamp: TimeTicks(11_932),
                variable_bindings: vec![
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 1, 3].into()),
                        value: TimeTicks(11_932).into(),
                    },
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 4, 1, 11779, 1, 42, 2, 1, 7].into()),
                        value: Gauge(1).into(),
                    },
                ],
            },
        };

        // TODO: Currently presence of any elements in `variable_bindings` throws a choice error.
        // Encoding succeeds and is correct with that field empty. There's a smoke-test for that
        // below for now.
        let encode_data = rasn::ber::encode(&encode_msg).unwrap();
        assert_eq!(encode_data, decode_data);

        let encode_msg_no_bindings = Message {
            data: Trap {
                variable_bindings: vec![],
                ..encode_msg.data
            },
            ..encode_msg
        };
        assert!(rasn::ber::encode(&encode_msg_no_bindings).is_ok());
    }
}
