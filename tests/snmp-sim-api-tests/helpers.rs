use crate::service_scope::ServiceScope;
use crate::test_app::TestApp;
use cancellation::CancellationTokenSource;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use snmp_sim::configuration::get_configuration;
use static_init::dynamic;
use std::io::{Error as IoError, ErrorKind};
use std::process::Child;
use std::process::Command;
use std::sync::Mutex;
use std::time::Duration;
use uuid_dev::Uuid;

// used to store the child process handle
lazy_static! {
    pub static ref SERVICE_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
}

// used to kill the child process, if created
struct TestRunGuard;

#[dynamic(drop)]
static mut TEST_RUN_GUARD: TestRunGuard = TestRunGuard {};

impl Drop for TestRunGuard {
    fn drop(&mut self) {
        let mut service_process = SERVICE_PROCESS.lock().unwrap();
        if service_process.is_some() {
            let _ = (*service_process).as_mut().unwrap().kill();
        }
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    if testing_service_endpoint_env().is_none() {
        // TESTING_ENV_SERVICE_URL is not defined => start a testing instance of the
        // service get the snmp_sim binary path
        let mut service_command = get_binary("snmp_sim").expect("snmp_sim binary not found");

        // spawn an instance of snmp_sim service
        let service_process = service_command
            //            .kill_on_drop(true)
            .spawn()
            .expect("Failed to start an instance of snmp_sim service");

        // store the child process handle => needs to be killed at the end of the test
        // run
        let mut value = SERVICE_PROCESS.lock().unwrap();
        *value = Some(service_process);
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

fn testing_service_endpoint() -> String {
    match testing_service_endpoint_env() {
        Some(var) => var,
        _ => {
            let config = get_configuration(None).expect("Failed to read configuration.");
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
    let address = testing_service_endpoint();

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
        let _ = create_agent(conn, &Uuid::new_v4(), &Uuid::new_v4().to_string())
            .await
            .unwrap()
            .unwrap_created();
    }
}
