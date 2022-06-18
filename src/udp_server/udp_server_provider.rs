use crate::domain::ManagedDevice;
use crate::snmp::handlers::snmp_generic_handler::generic_snmp_message_handler;
use crate::udp_server::udp_server_error::UdpServerError;
use crate::udp_server::udp_stream_handler::StopActor;
use crate::udp_server::udp_stream_handler::UdpStreamHandler;
use actix_async::address::Addr;
use actix_async::prelude::*;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid_dev::Uuid;

/// UDP Server implementation which supports handling of multiple UDP listeners
///
/// The UDP server starts without no active listener. New listener
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpServerProvider {
    // we start only one instance of UdpServerProvider => access to the resource doesn't need to be mutexed
    udp_stream_handlers: RwLock<HashMap<Uuid, Addr<UdpStreamHandler>>>,
}

impl UdpServerProvider {
    pub fn new() -> Self {
        UdpServerProvider {
            udp_stream_handlers: RwLock::new(HashMap::new()),
        }
    }
}

actor!(UdpServerProvider);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct StartSnmpDevice {
    pub device: ManagedDevice,
}
message!(StartSnmpDevice, Result<(), UdpServerError>);

#[actix_async::handler]
impl Handler<StartSnmpDevice> for UdpServerProvider {
    #[tracing::instrument(level = "info", name = "UdpServerProvider::StartSnmpDevice", skip(self, _ctx))]
    async fn handle(&self, msg: StartSnmpDevice, _ctx: Context<'_, Self>) -> Result<(), UdpServerError> {
        if self
            .udp_stream_handlers
            .read()
            .await
            .contains_key(&msg.device.id)
        {
            Err(UdpServerError::DeviceAlreadyRunning)
        } else {
            let device_id = msg.device.id;
            let udp_stream_handler_addr = UdpStreamHandler::new(&generic_snmp_message_handler, msg.device).await?;
            self.udp_stream_handlers
                .write()
                .await
                .insert(device_id, udp_stream_handler_addr);

            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct StopSnmpDevice {
    pub device_id: Uuid,
}
message!(StopSnmpDevice, Result<(), UdpServerError>);

#[actix_async::handler]
impl Handler<StopSnmpDevice> for UdpServerProvider {
    #[tracing::instrument(level = "info", name = "UdpServerProvider::StopSnmpDevice", skip(self, _ctx))]
    async fn handle(&self, msg: StopSnmpDevice, _ctx: Context<'_, Self>) -> Result<(), UdpServerError> {
        if let Some(addr) = self
            .udp_stream_handlers
            .write()
            .await
            .remove(&msg.device_id)
        {
            // device is running, send a message to the actor to stop message handling
            if let Err(error) = addr.send(StopActor {}).await {
                tracing::error!("{error}");
            }
            Ok(())
        } else {
            return Err(UdpServerError::DeviceNotRunning);
        }
    }
}
