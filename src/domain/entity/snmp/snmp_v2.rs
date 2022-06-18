use crate::domain::try_to_i32;
use crate::domain::{GetNextRequest, GetRequest, GetResponse, GetResponseError, ValidationError, Variable};
use num_traits::ToPrimitive;
use num_traits::Zero;
use rasn_smi::v1::Gauge;
use rasn_smi::v1::IpAddress;
use rasn_smi::v2::*;
use rasn_snmp::v2::VarBindValue;
use snmp_data_parser::parser::snmp_data::component::string_to_oid;
use snmp_data_parser::parser::snmp_data::component::DataType;

impl TryFrom<rasn_snmp::v2::GetRequest> for GetRequest {
    type Error = ValidationError;

    fn try_from(rasn_req: rasn_snmp::v2::GetRequest) -> Result<Self, Self::Error> {
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

impl TryFrom<rasn_snmp::v2::GetNextRequest> for GetNextRequest {
    type Error = ValidationError;

    fn try_from(rasn_req: rasn_snmp::v2::GetNextRequest) -> Result<Self, Self::Error> {
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

impl From<GetRequest> for rasn_snmp::v2::Pdu {
    fn from(request: GetRequest) -> Self {
        rasn_snmp::v2::Pdu {
            request_id: request.request_id.to_i32().unwrap(),
            error_status: rasn_snmp::v2::Pdu::ERROR_STATUS_NO_ERROR,
            error_index: Zero::zero(),
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
                .map(rasn_snmp::v2::VarBind::from)
                .collect(),
        }
    }
}

impl From<GetNextRequest> for rasn_snmp::v2::Pdu {
    fn from(request: GetNextRequest) -> Self {
        rasn_snmp::v2::Pdu {
            request_id: request.request_id.to_i32().unwrap(),
            error_status: rasn_snmp::v2::Pdu::ERROR_STATUS_NO_ERROR,
            error_index: Zero::zero(),
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
                .map(rasn_snmp::v2::VarBind::from)
                .collect(),
        }
    }
}

impl From<GetResponse> for rasn_snmp::v2::Pdu {
    fn from(response: GetResponse) -> Self {
        rasn_snmp::v2::Pdu {
            request_id: response.request_id.to_i32().unwrap(),
            error_status: rasn_snmp::v2::Pdu::ERROR_STATUS_NO_ERROR,
            error_index: Zero::zero(),
            variable_bindings: response
                .variable_values
                .iter()
                .map(rasn_snmp::v2::VarBind::from)
                .collect(),
        }
    }
}

impl From<GetRequest> for rasn_snmp::v2::Pdus {
    fn from(response: GetRequest) -> Self {
        rasn_snmp::v2::Pdus::GetRequest(rasn_snmp::v2::GetRequest(response.into()))
    }
}

impl From<GetNextRequest> for rasn_snmp::v2::Pdus {
    fn from(response: GetNextRequest) -> Self {
        rasn_snmp::v2::Pdus::GetNextRequest(rasn_snmp::v2::GetNextRequest(response.into()))
    }
}

impl From<GetResponse> for rasn_snmp::v2::Pdus {
    fn from(response: GetResponse) -> Self {
        rasn_snmp::v2::Pdus::Response(rasn_snmp::v2::Response(response.into()))
    }
}

impl From<GetResponseError> for rasn_snmp::v2::Pdus {
    fn from(response: GetResponseError) -> Self {
        rasn_snmp::v2::Pdus::Response(rasn_snmp::v2::Response(response.into()))
    }
}

impl From<GetResponseError> for rasn_snmp::v2::Pdu {
    fn from(response: GetResponseError) -> Self {
        let variable_bindings = match response.name {
            Some(name) => vec![rasn_snmp::v2::VarBind {
                name,
                value: VarBindValue::Unspecified,
            }],
            _ => vec![],
        };

        rasn_snmp::v2::Pdu {
            request_id: response.request_id.to_i32().unwrap(),
            error_status: (response.error_status as u32),
            error_index: response.error_index.to_u32().unwrap(),
            variable_bindings,
        }
    }
}

impl From<&Variable> for rasn_snmp::v2::VarBind {
    fn from(variable: &Variable) -> Self {
        let value = match variable.data_type {
            DataType::String => VarBindValue::Value(ObjectSyntax::Simple(SimpleSyntax::String(
                variable.value.clone().into(),
            ))),
            DataType::Oid => VarBindValue::Value(ObjectSyntax::Simple(SimpleSyntax::ObjectId(string_to_oid(
                &variable.value,
            )))),
            DataType::Null => VarBindValue::Unspecified,
            DataType::Integer => VarBindValue::Value(ObjectSyntax::Simple(SimpleSyntax::Integer(
                variable.value.clone().parse().unwrap(),
            ))),
            DataType::UInteger32 => VarBindValue::Value(ObjectSyntax::ApplicationWide(ApplicationSyntax::Unsigned(
                rasn_smi::v1::Gauge(variable.value.clone().parse().unwrap()),
            ))),
            DataType::Counter32 => {
                let value: u32 = variable.value.clone().parse().unwrap();
                VarBindValue::Value(ObjectSyntax::ApplicationWide(ApplicationSyntax::Counter(
                    rasn_smi::v1::Counter(value),
                )))
            }
            DataType::Counter64 => {
                let value: u64 = variable.value.clone().parse().unwrap();
                VarBindValue::Value(ObjectSyntax::ApplicationWide(ApplicationSyntax::BigCounter(Counter64(
                    value,
                ))))
            }
            DataType::Gauge32 => {
                let value: u32 = variable.value.clone().parse().unwrap();
                VarBindValue::Value(ObjectSyntax::ApplicationWide(ApplicationSyntax::Unsigned(Gauge(value))))
            }
            DataType::IpAddress => {
                use std::net::Ipv4Addr;
                let value: Ipv4Addr = variable.value.clone().parse().unwrap();
                VarBindValue::Value(ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(IpAddress(
                    bytes::Bytes::from(value.octets().to_vec()),
                ))))
            }
            _ => VarBindValue::Value(ObjectSyntax::Simple(SimpleSyntax::String(
                variable.value.clone().into(),
            ))),
            // DataType::HexString => {}
            // DataType::Timeticks => {}
            // DataType::NetworkAddress => {}
            // DataType::Bits => {}
            // DataType::Opaque => {}
            // DataType::OctetString => {}
        };

        rasn_snmp::v2::VarBind {
            name: variable.name.clone(),
            value,
        }
    }
}
