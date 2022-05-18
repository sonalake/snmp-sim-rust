use crate::domain::ManagedDevice;
use crate::udp_server::udp_server_error::UdpServerError;
use crate::udp_server::udp_server_provider::{StartSnmpDevice, StopSnmpDevice, UdpServerProvider};
use actix::Addr;

#[derive(Clone)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UdpServerDelegate {
    service_config_provider: Addr<UdpServerProvider>,
}

impl UdpServerDelegate {
    pub fn new(service_config_provider: Addr<UdpServerProvider>) -> Self {
        Self {
            service_config_provider,
        }
    }

    #[tracing::instrument(level = "info", name = "UdpServerDelegate::start_snmp_device", skip(self, device))]
    pub async fn start_snmp_device(&self, device: ManagedDevice) -> Result<(), UdpServerError> {
        start_snmp_device(self.service_config_provider.clone(), device).await
    }

    #[tracing::instrument(level = "info", name = "UdpServerDelegate::stop_snmp_device", skip(self, device))]
    pub async fn stop_snmp_device(&self, device: ManagedDevice) -> Result<(), UdpServerError> {
        stop_snmp_device(self.service_config_provider.clone(), device).await
    }
}

#[tracing::instrument(level = "info", name = "start_snmp_device", skip(service_config_provider, device))]
async fn start_snmp_device(
    service_config_provider: Addr<UdpServerProvider>,
    device: ManagedDevice,
) -> Result<(), UdpServerError> {
    service_config_provider
        .send(StartSnmpDevice { device })
        .await?
}

#[tracing::instrument(level = "info", name = "stop_snmp_device", skip(service_config_provider, device))]
async fn stop_snmp_device(
    service_config_provider: Addr<UdpServerProvider>,
    device: ManagedDevice,
) -> Result<(), UdpServerError> {
    service_config_provider
        .send(StopSnmpDevice { device_id: device.id })
        .await?
}
