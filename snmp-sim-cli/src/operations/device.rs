use crate::cli::CliContext;
use crate::subcommands::device::{CreateDevice, Device, UpdateDevice};
use rust_client_snmp_sim_lib::apis::configuration::Configuration;
use rust_client_snmp_sim_lib::apis::devices_api::*;
use rust_client_snmp_sim_lib::models::RequestDevice;
use tracing::{self, trace};

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn list_devices(ctx: &CliContext<'_>) -> Result<(), anyhow::Error> {
    trace!("List all devices");
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let devices = devices_get(&configuration, None, None).await?;
    for device in devices.iter() {
        println!("{:#?}", device);
    }

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn create_device(ctx: &CliContext<'_>, create_device: CreateDevice) -> Result<(), anyhow::Error> {
    trace!("Create a new instance of device={:#?}", create_device);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let mut device = RequestDevice::new(
        create_device.agent_id,
        create_device.name,
        serde_json::from_str(&create_device.protocol).unwrap(),
    );
    device.description = create_device.description;

    let created_device = devices_post(&configuration, device).await?;
    println!("{:#?}", created_device);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn update_device(ctx: &CliContext<'_>, update_device: UpdateDevice) -> Result<(), anyhow::Error> {
    trace!("Update an existing device={:?}", update_device);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let mut device = RequestDevice::new(
        update_device.agent_id,
        update_device.name,
        serde_json::from_str(&update_device.protocol).unwrap(),
    );
    device.description = update_device.description;

    let updated_device = devices_id_put(&configuration, &update_device.id, device).await?;
    println!("{:#?}", updated_device);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn delete_device(ctx: &CliContext<'_>, device: Device) -> Result<(), anyhow::Error> {
    trace!("Delete an existing device={:?}", device);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let deleted_device = devices_id_delete(&configuration, &device.id).await?;
    println!("{:#?}", deleted_device);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn get_device(ctx: &CliContext<'_>, device: Device) -> Result<(), anyhow::Error> {
    trace!("Delete an existing device={:?}", device);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let device = devices_id_get(&configuration, &device.id).await?;
    println!("{:#?}", device);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn start_device(ctx: &CliContext<'_>, device: Device) -> Result<(), anyhow::Error> {
    trace!("Start an existing device={:?}", device);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let device = devices_id_start_put(&configuration, &device.id).await?;
    println!("{:#?}", device);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn stop_device(ctx: &CliContext<'_>, device: Device) -> Result<(), anyhow::Error> {
    trace!("Stop an existing device={:?}", device);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let device = devices_id_stop_put(&configuration, &device.id).await?;
    println!("{:#?}", device);

    Ok(())
}
