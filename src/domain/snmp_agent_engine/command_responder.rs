use crate::domain::{ErrorStatus, GetNextRequest, GetRequest, GetResponse, GetResponseError, Variable};
use crate::snmp::codec::generic_snmp_message::GenericSnmpMessage;
use crate::snmp::handlers::snmp_generic_handler::RequestContext;
use crate::udp_server::udp_stream_handler::send_data;

use actix::prelude::*;
use rasn::prelude::ObjectIdentifier;
use shared_common::error_chain_fmt;
use snmp_data_parser::parser::ParserError;
use std::convert::Infallible;
use std::ops::Bound::{Excluded, Unbounded};

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(thiserror::Error)]
pub(crate) enum SnmpAgentCommandResponderError {
    #[error("ErrorStatus={0} Index={1}")]
    ProtocolError(ErrorStatus, usize, ObjectIdentifier),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    Parser(#[from] ParserError),

    #[error("{0}")]
    SendError(String),
}

impl std::fmt::Debug for SnmpAgentCommandResponderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for SnmpAgentCommandResponderError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to SnmpAgentCommandResponderError")
    }
}

// Implementation of SNMP agent command responder actor
#[derive(Default)]
struct SnmpAgentCommandResponder;

impl Actor for SnmpAgentCommandResponder {
    type Context = Context<Self>;
}

impl Supervised for SnmpAgentCommandResponder {}
impl SystemService for SnmpAgentCommandResponder {}

// SNMP GetRequest Handler
#[derive(Message, Debug)]
#[rtype(result = "Result<(), SnmpAgentCommandResponderError>")]
struct Get {
    pub request: GetRequest,
    pub request_context: RequestContext,
}

impl Handler<Get> for SnmpAgentCommandResponder {
    type Result = Result<(), SnmpAgentCommandResponderError>;

    #[tracing::instrument(
        level = "info",
        name = "SnmpAgentCommandResponder::handle_get_request",
        skip(self, get_msg, _ctx)
    )]
    fn handle(&mut self, get_msg: Get, _ctx: &mut Context<Self>) -> Self::Result {
        let get_request = get_msg.request;
        let variables = get_request
            .objects
            .into_iter()
            .enumerate()
            .map(|(idx, item)| match get_msg.request_context.snmp_data.get(&item) {
                Some(snmp_data_item) => Ok(Variable {
                    name: item,
                    data_type: snmp_data_item.data_type.clone(),
                    value: snmp_data_item.data_value.clone(),
                }),
                _ => Err(SnmpAgentCommandResponderError::ProtocolError(
                    ErrorStatus::NoSuchName,
                    idx + 1,
                    item,
                )),
            })
            .collect();

        let response: GenericSnmpMessage = match variables {
            Ok(variable_values) => {
                let response = GetResponse {
                    request_id: get_request.request_id,
                    variable_values,
                };
                (&get_msg.request_context.version, response).into()
            }
            Err(SnmpAgentCommandResponderError::ProtocolError(error_status, error_index, name)) => {
                let response = GetResponseError {
                    request_id: get_request.request_id,
                    error_status,
                    error_index,
                    name: Some(name),
                };
                (&get_msg.request_context.version, response).into()
            }
            _ => {
                let response = GetResponseError {
                    request_id: get_request.request_id,
                    error_status: ErrorStatus::GenErr,
                    error_index: 0,
                    name: None,
                };
                (&get_msg.request_context.version, response).into()
            }
        };

        send_data(response, &get_msg.request_context);

        Ok(())
    }
}

// SNMP GetNextRequest Handler
#[derive(Message, Debug)]
#[rtype(result = "Result<(), SnmpAgentCommandResponderError>")]
struct GetNext {
    pub request: GetNextRequest,
    pub request_context: RequestContext,
}

impl Handler<GetNext> for SnmpAgentCommandResponder {
    type Result = Result<(), SnmpAgentCommandResponderError>;

    #[tracing::instrument(
        level = "info",
        name = "SnmpAgentCommandResponder::handle_get_next_request",
        skip(self, get_next_msg, _ctx)
    )]
    fn handle(&mut self, get_next_msg: GetNext, _ctx: &mut Context<Self>) -> Self::Result {
        let get_next_request = get_next_msg.request;
        let variables = get_next_request
            .objects
            .into_iter()
            .enumerate()
            .map(|(idx, item)| {
                match get_next_msg
                    .request_context
                    .snmp_data
                    .range((Excluded(item.clone()), Unbounded))
                    .next()
                {
                    Some((name, snmp_data_item)) => Ok(Variable {
                        name: name.clone(),
                        data_type: snmp_data_item.data_type.clone(),
                        value: snmp_data_item.data_value.clone(),
                    }),
                    _ => Err(SnmpAgentCommandResponderError::ProtocolError(
                        ErrorStatus::NoSuchName,
                        idx + 1,
                        item,
                    )),
                }
            })
            .collect();

        let response: GenericSnmpMessage = match variables {
            Ok(variable_values) => {
                let response = GetResponse {
                    request_id: get_next_request.request_id,
                    variable_values,
                };
                (&get_next_msg.request_context.version, response).into()
            }
            Err(SnmpAgentCommandResponderError::ProtocolError(error_status, error_index, name)) => {
                let response = GetResponseError {
                    request_id: get_next_request.request_id,
                    error_status,
                    error_index,
                    name: Some(name),
                };
                (&get_next_msg.request_context.version, response).into()
            }
            _ => {
                let response = GetResponseError {
                    request_id: get_next_request.request_id,
                    error_status: ErrorStatus::GenErr,
                    error_index: 0,
                    name: None,
                };
                (&get_next_msg.request_context.version, response).into()
            }
        };

        send_data(response, &get_next_msg.request_context);

        Ok(())
    }
}

// delegates
#[tracing::instrument(level = "info", name = "handle_get_request")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn handle_get_request(
    request: GetRequest,
    request_context: RequestContext,
) -> Result<(), SnmpAgentCommandResponderError> {
    tracing::info!("Get SnmpAgentCommandResponder from registry");
    let act = SnmpAgentCommandResponder::from_registry();

    tracing::info!("Sending message to SnmpAgentCommandResponder actor");
    act.try_send(Get {
        request,
        request_context,
    })
    .map_err(|err| SnmpAgentCommandResponderError::SendError(err.to_string()))
}

#[tracing::instrument(level = "info", name = "handle_get_next_request")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn handle_get_next_request(
    request: GetNextRequest,
    request_context: RequestContext,
) -> Result<(), SnmpAgentCommandResponderError> {
    let act = SnmpAgentCommandResponder::from_registry();
    act.try_send(GetNext {
        request,
        request_context,
    })
    .map_err(|err| SnmpAgentCommandResponderError::SendError(err.to_string()))
}
