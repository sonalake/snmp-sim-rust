use crate::service_scope::ServiceScope;
use crate::test_app::TestApp;
use cancellation::CancellationTokenSource;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use signal_child::Signalable;
use snmp_sim::configuration::get_configuration;
use snmp_sim::configuration::Settings;
use static_init::dynamic;
use std::io::{Error as IoError, ErrorKind};
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::Duration;
use uuid_dev::Uuid;

// used to store the child process handle
lazy_static! {
    pub static ref SERVICE_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
}

#[cfg(test)]
#[ctor::ctor]
fn setup_service_singleton() {
    if testing_service_endpoint_env().is_none() {
        let mut value = SERVICE_PROCESS.lock().unwrap();
        if (*value).is_none() {
            // TESTING_ENV_SERVICE_URL is not defined => start a testing instance of the service

            // get the snmp_sim binary path
            let mut service_command = get_binary("snmp_sim").expect("snmp_sim binary not found");

            // spawn an instance of snmp_sim service
            let service_process = service_command
                // disabled to avoid tarpaulin hang
                // .kill_on_drop(true)
                .spawn()
                .expect("Failed to start an instance of snmp_sim service");

            // store the child process handle => needs to be killed at the end of the test run
            *value = Some(service_process);
        }
    }
}

// used to kill the child process, if created
struct TestRunGuard;

#[dynamic(drop)]
static mut TEST_RUN_GUARD: TestRunGuard = TestRunGuard {};

impl Drop for TestRunGuard {
    fn drop(&mut self) {
        let mut service_process = SERVICE_PROCESS.lock().unwrap();
        if let Some(child) = service_process.as_mut() {
            let _ = child.term();
            let _ = child.wait();
            (*service_process) = None;
        }
    }
}

pub async fn spawn_app() -> TestApp {
    let service_scope = setup_service().await;

    TestApp::new(&service_scope.address, &service_scope.config.database).await
}

fn get_binary(bin_name: &str) -> Result<Command, IoError> {
    let current_exe = std::env::current_exe().expect("Failed to get the path of the integration test binary");
    let mut bin_dir = current_exe
        .parent()
        .expect("failed to get parent")
        .to_owned();
    bin_dir.pop();
    bin_dir.push(bin_name);
    bin_dir.set_extension(std::env::consts::EXE_EXTENSION);

    tracing::debug!("try to get binary: {:#?}", bin_dir);
    if !bin_dir.exists() {
        Err(IoError::new(
            ErrorKind::NotFound,
            format!("{} not found in: {:#?}", bin_name, bin_dir),
        ))
    } else {
        Ok(Command::new(bin_dir.into_os_string()))
    }
}

fn testing_service_endpoint(config: &Settings) -> String {
    match testing_service_endpoint_env() {
        Some(var) => var,
        _ => {
            format!("http://{}:{}", config.application.host, config.application.port)
        }
    }
}

fn testing_service_endpoint_env() -> Option<String> {
    let testing_env_var = std::env::var("TESTING_ENV_SERVICE_URL");
    match testing_env_var {
        Ok(var) => match var.len() {
            0 => None,
            _ => Some(var),
        },
        _ => None,
    }
}

async fn setup_service() -> ServiceScope {
    let config = get_configuration(None).expect("Failed to read configuration.");
    let address = testing_service_endpoint(&config);

    let cts = CancellationTokenSource::new();
    cts.cancel_after(Duration::from_millis(20000));

    // wait for service to boot
    ServiceScope {
        address: address.clone(),
        config,
    }
    .wait_service_running(&cts)
    .await
    .expect("Service failed to boot")
}

pub async fn seed_agents(conn: &DatabaseConnection, agents_count: usize) {
    use snmp_sim::data_access::helpers::*;
    for _ in 0..agents_count {
        let _ = create_agent(
            conn,
            &Uuid::new_v4(),
            &Uuid::new_v4().to_string(),
            &Some(Uuid::new_v4().to_string()),
            &Uuid::new_v4().to_string(),
        )
        .await;
    }
}

// Creates required number device instances and returns the list of their unique identifiers
pub async fn seed_devices(
    conn: &DatabaseConnection,
    agent_id: &Uuid,
    devices_count: usize,
    snmp_host: &str,
    snmp_initial_port: u16,
) -> std::vec::Vec<Uuid> {
    use snmp_sim::data_access::helpers::*;

    futures::future::join_all((0..devices_count).map(|idx| async move {
        let device_id = Uuid::new_v4();
        let device_name = Uuid::new_v4().to_string();
        let device_description = Some(Uuid::new_v4().to_string());
        let protocol = domain_snmp_v1_attributes_json("public");
        let snmp_port = snmp_initial_port + idx as u16;
        create_managed_device(
            conn,
            &device_id,
            &device_name,
            &device_description,
            agent_id,
            &protocol,
            snmp_host,
            snmp_port,
        )
        .await
    }))
    .await
    .into_iter()
    .filter_map(|res| match res {
        Ok(item) if item.is_created() => {
            let (device, _agent) = item.unwrap_created();
            Some(Uuid::parse_str(&device.id).unwrap())
        }
        _ => None,
    })
    .collect()
}

//----- SNMP V1 protocol attributes for route and domain layers, raw and json format
pub fn route_snmp_v1_attributes(community: &str) -> snmp_sim::routes::SnmpProtocolAttributes {
    snmp_sim::routes::SnmpProtocolAttributes {
        snmp_v1: Some(snmp_sim::routes::SnmpV1Attributes {
            community: Some(community.to_string()),
        }),
        snmp_v2c: None,
        snmp_v3: None,
    }
}

pub fn domain_snmp_v1_attributes(community: &str) -> snmp_sim::domain::SnmpProtocolAttributes {
    snmp_sim::domain::SnmpProtocolAttributes::SnmpV1(snmp_sim::domain::SnmpV1Attributes {
        community: community.to_string(),
    })
}

pub fn domain_snmp_v1_attributes_json(community: &str) -> String {
    serde_json::to_string(&domain_snmp_v1_attributes(community)).unwrap()
}

//----- SNMP V2c protocol attributes for route and domain layers, raw and json format
pub fn route_snmp_v2c_attributes(community: &str) -> snmp_sim::routes::SnmpProtocolAttributes {
    snmp_sim::routes::SnmpProtocolAttributes {
        snmp_v1: None,
        snmp_v2c: Some(snmp_sim::routes::SnmpV2cAttributes {
            community: Some(community.to_string()),
        }),
        snmp_v3: None,
    }
}

pub fn domain_snmp_v2c_attributes(community: &str) -> snmp_sim::domain::SnmpProtocolAttributes {
    snmp_sim::domain::SnmpProtocolAttributes::SnmpV2c(snmp_sim::domain::SnmpV2cAttributes {
        community: community.to_string(),
    })
}

pub fn domain_snmp_v2c_attributes_json(community: &str) -> String {
    serde_json::to_string(&domain_snmp_v2c_attributes(community)).unwrap()
}

//----- SNMP V3 protocol attributes for route and domain layers, raw and json format
pub fn _snmp_v3_attributes(
    user: &str,
    auth_alg: snmp_sim::routes::AuthenticationAlgorithm,
    auth_key: &str,
    enc_alg: snmp_sim::routes::EncryptionAlgorithm,
    enc_key: &str,
) -> snmp_sim::routes::SnmpProtocolAttributes {
    snmp_sim::routes::SnmpProtocolAttributes {
        snmp_v1: None,
        snmp_v2c: None,
        snmp_v3: Some(snmp_sim::routes::SnmpV3Attributes {
            user: Some(user.to_string()),
            authentication: Some(auth_alg),
            authentication_password: auth_key.to_string(),
            encryption: Some(enc_alg),
            encryption_key: enc_key.to_string(),
        }),
    }
}
