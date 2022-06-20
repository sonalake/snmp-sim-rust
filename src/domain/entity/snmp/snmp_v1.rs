use crate::domain::try_to_i32;
use crate::domain::{GetNextRequest, GetRequest, GetResponse, GetResponseError, ValidationError, Variable};
use num_bigint::ToBigInt;
use rasn_smi::v1::*;
use snmp_data_parser::parser::snmp_data::component::string_to_oid;
use snmp_data_parser::parser::snmp_data::component::DataType;

impl TryFrom<rasn_snmp::v1::GetRequest> for GetRequest {
    type Error = ValidationError;

    fn try_from(rasn_req: rasn_snmp::v1::GetRequest) -> Result<Self, Self::Error> {
        let pdu = rasn_req.0;
        let objects = pdu
            .variable_bindings
            .into_iter()
            .map(|item| item.name)
            .collect();
        Ok(GetRequest {
            request_id: try_to_i32(&pdu.request_id)?,
            objects,
        })
    }
}

impl TryFrom<rasn_snmp::v1::GetNextRequest> for GetNextRequest {
    type Error = ValidationError;

    fn try_from(rasn_req: rasn_snmp::v1::GetNextRequest) -> Result<Self, Self::Error> {
        let pdu = rasn_req.0;
        let objects = pdu
            .variable_bindings
            .iter()
            .map(|item| item.name.clone())
            .collect();
        Ok(GetNextRequest {
            request_id: try_to_i32(&pdu.request_id)?,
            objects,
        })
    }
}

impl From<GetRequest> for rasn_snmp::v1::Pdu {
    fn from(request: GetRequest) -> Self {
        rasn_snmp::v1::Pdu {
            request_id: request.request_id.to_bigint().unwrap(),
            error_status: rasn_snmp::v1::Pdu::ERROR_STATUS_NO_ERROR.into(),
            error_index: 0.into(),
            variable_bindings: request
                .objects
                .into_iter()
                .map(|oid| Variable {
                    name: oid,
                    data_type: DataType::Null,
                    value: String::default(),
                })
                .collect::<Vec<Variable>>()
                .iter()
                .map(rasn_snmp::v1::VarBind::from)
                .collect(),
        }
    }
}

impl From<GetNextRequest> for rasn_snmp::v1::Pdu {
    fn from(request: GetNextRequest) -> Self {
        rasn_snmp::v1::Pdu {
            request_id: request.request_id.to_bigint().unwrap(),
            error_status: rasn_snmp::v1::Pdu::ERROR_STATUS_NO_ERROR.into(),
            error_index: 0.into(),
            variable_bindings: request
                .objects
                .into_iter()
                .map(|oid| Variable {
                    name: oid,
                    data_type: DataType::Null,
                    value: String::default(),
                })
                .collect::<Vec<Variable>>()
                .iter()
                .map(rasn_snmp::v1::VarBind::from)
                .collect(),
        }
    }
}

impl From<GetResponse> for rasn_snmp::v1::Pdu {
    fn from(response: GetResponse) -> Self {
        rasn_snmp::v1::Pdu {
            request_id: response.request_id.to_bigint().unwrap(),
            error_status: rasn_snmp::v1::Pdu::ERROR_STATUS_NO_ERROR.into(),
            error_index: 0.into(),
            variable_bindings: response
                .variable_values
                .iter()
                .map(rasn_snmp::v1::VarBind::from)
                .collect(),
        }
    }
}

impl From<GetResponseError> for rasn_snmp::v1::Pdu {
    fn from(response: GetResponseError) -> Self {
        let variable_bindings = match response.name {
            Some(name) => vec![rasn_snmp::v1::VarBind {
                name,
                value: ObjectSyntax::Simple(SimpleSyntax::Empty),
            }],
            _ => vec![],
        };

        rasn_snmp::v1::Pdu {
            request_id: response.request_id.to_bigint().unwrap(),
            error_status: (response.error_status as u32).into(),
            error_index: response.error_index.into(),
            variable_bindings,
        }
    }
}

impl From<&Variable> for rasn_snmp::v1::VarBind {
    fn from(variable: &Variable) -> Self {
        let value = match variable.value.as_str() {
            "" => ObjectSyntax::Simple(SimpleSyntax::Empty),
            _ => match variable.data_type {
                DataType::String => ObjectSyntax::Simple(SimpleSyntax::String(variable.value.clone().into())),
                DataType::Oid => ObjectSyntax::Simple(SimpleSyntax::Object(string_to_oid(&variable.value))),
                DataType::Null => ObjectSyntax::Simple(SimpleSyntax::Empty),
                DataType::Integer => {
                    ObjectSyntax::Simple(SimpleSyntax::Number(variable.value.clone().parse().unwrap()))
                }
                DataType::UInteger32 => {
                    ObjectSyntax::Simple(SimpleSyntax::Number(variable.value.clone().parse().unwrap()))
                }
                DataType::Counter32 => {
                    let value: u32 = variable.value.clone().parse().unwrap();
                    ObjectSyntax::ApplicationWide(ApplicationSyntax::Counter(Counter(value)))
                }
                DataType::Counter64 => {
                    if let Ok(value) = variable.value.clone().parse() {
                        ObjectSyntax::ApplicationWide(ApplicationSyntax::Counter(Counter(value)))
                    } else {
                        ObjectSyntax::Simple(SimpleSyntax::String(variable.value.clone().into()))
                    }
                }
                DataType::Gauge32 => {
                    let value: u32 = variable.value.clone().parse().unwrap();
                    ObjectSyntax::ApplicationWide(ApplicationSyntax::Gauge(Gauge(value)))
                }
                DataType::IpAddress => {
                    use std::net::Ipv4Addr;
                    let value: Ipv4Addr = variable.value.clone().parse().unwrap();
                    ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(NetworkAddress::Internet(IpAddress(
                        bytes::Bytes::from(value.octets().to_vec()),
                    ))))
                }

                _ => ObjectSyntax::Simple(SimpleSyntax::String(variable.value.clone().into())),
                // DataType::NetworkAddress => ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(
                //     NetworkAddress::Internet(IpAddress(variable.value.clone().into())),
                // )),
                // DataType::HexString => {}
                // DataType::Timeticks => {}
                // DataType::Bits => {}
                // DataType::Opaque => {}
                // DataType::OctetString => {}
            },
        };

        rasn_snmp::v1::VarBind {
            name: variable.name.clone(),
            value,
        }
    }
}

impl From<GetRequest> for rasn_snmp::v1::Pdus {
    fn from(request: GetRequest) -> Self {
        rasn_snmp::v1::Pdus::GetRequest(rasn_snmp::v1::GetRequest(request.into()))
    }
}

impl From<GetNextRequest> for rasn_snmp::v1::Pdus {
    fn from(request: GetNextRequest) -> Self {
        rasn_snmp::v1::Pdus::GetNextRequest(rasn_snmp::v1::GetNextRequest(request.into()))
    }
}

impl From<GetResponse> for rasn_snmp::v1::Pdus {
    fn from(response: GetResponse) -> Self {
        rasn_snmp::v1::Pdus::GetResponse(rasn_snmp::v1::GetResponse(response.into()))
    }
}

impl From<GetResponseError> for rasn_snmp::v1::Pdus {
    fn from(response: GetResponseError) -> Self {
        rasn_snmp::v1::Pdus::GetResponse(rasn_snmp::v1::GetResponse(response.into()))
    }
}
