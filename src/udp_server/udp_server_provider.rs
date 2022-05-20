use crate::domain::ManagedDevice;
use crate::snmp::handlers::snmp_generic_handler::generic_snmp_message_handler;
use crate::udp_server::udp_server_error::UdpServerError;
use crate::udp_server::udp_stream_handler::StopActor;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;
use actix::prelude::*;
use actix::{Actor, Context};
use std::collections::HashMap;
use uuid_dev::Uuid;

/// UDP Server implementation which supports handling of multiple UDP listeners
///
/// The UDP server starts without no active listener. New listener
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpServerProvider {
    // we start only one instance of UdpServerProvider => access to the resource doesn't need to be mutexed
    udp_stream_handlers: HashMap<Uuid, Addr<UdpStreamHandler>>,
}

impl UdpServerProvider {
    pub fn new() -> Self {
        UdpServerProvider {
            udp_stream_handlers: HashMap::new(),
        }
    }
}

impl Actor for UdpServerProvider {
    type Context = Context<Self>;

    #[tracing::instrument(level = "info", name = "UdpServerProvider::started", skip(self, _ctx))]
    fn started(&mut self, _ctx: &mut Self::Context) {}
}

// implement the Supervised trait to create a new execution context
// and restart the actor’s lifecycle in case of actor failure
impl actix::Supervised for UdpServerProvider {
    #[tracing::instrument(level = "info", name = "UdpServerProvider::restarting", skip(self, _ctx))]
    fn restarting(&mut self, _ctx: &mut Context<Self>) {}
}

#[derive(Clone, Debug, Message)]
#[rtype("Result<(), UdpServerError>")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct StartSnmpDevice {
    pub device: ManagedDevice,
}

impl Handler<StartSnmpDevice> for UdpServerProvider {
    type Result = Result<(), UdpServerError>;

    #[tracing::instrument(level = "info", name = "UdpServerProvider::StartSnmpDevice", skip(self, _ctx))]
    fn handle(&mut self, msg: StartSnmpDevice, _ctx: &mut Self::Context) -> Self::Result {
        if self.udp_stream_handlers.contains_key(&msg.device.id) {
            return Err(UdpServerError::DeviceAlreadyRunning);
        }

        let device_id = msg.device.id;
        let udp_stream_handler = UdpStreamHandler::new(&generic_snmp_message_handler, msg.device)?;
        let addr = actix::Supervisor::start(move |_| udp_stream_handler);

        self.udp_stream_handlers.insert(device_id, addr);

        Ok(())
    }
}

#[derive(Clone, Debug, Message)]
#[rtype("Result<(), UdpServerError>")]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct StopSnmpDevice {
    pub device_id: Uuid,
}

impl Handler<StopSnmpDevice> for UdpServerProvider {
    type Result = Result<(), UdpServerError>;

    #[tracing::instrument(level = "info", name = "UdpServerProvider::StopSnmpDevice", skip(self, _ctx))]
    fn handle(&mut self, msg: StopSnmpDevice, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.udp_stream_handlers.remove(&msg.device_id) {
            // device is running, send a message to the actor to stop message handling
            if let Err(error) = addr.try_send(StopActor {}) {
                tracing::error!("{error}");
            }
        } else {
            return Err(UdpServerError::DeviceNotRunning);
        }

        Ok(())
    }
}
