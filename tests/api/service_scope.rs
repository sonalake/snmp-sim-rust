use crate::helpers::SERVICE_PROCESS;
use cancellation::{CancellationToken, OperationCanceled};
use snmp_sim::configuration::Settings;
use std::time::Duration;

#[derive(Clone)]
pub struct ServiceScope {
    pub address: String,
    pub config: Settings,
}

impl ServiceScope {
    pub async fn wait_service_running(self, cts: &CancellationToken) -> Result<Self, OperationCanceled> {
        self.wait_service_request_status_ok("/agents?page=1&page_size=1", cts)
            .await?;

        Ok(self)
    }

    async fn wait_service_request_status_ok(
        &self,
        endpoint: &str,
        cts: &CancellationToken,
    ) -> Result<(), OperationCanceled> {
        loop {
            // exit function if the cancellation token has expired
            cts.result()?;

            // exit function if the service is not running
            if !is_service_process_running() {
                return Err(OperationCanceled);
            }

            // send service request query to check the service readyness
            if let Some(reqwest::StatusCode::OK) = self.service_request_status(endpoint).await {
                return Ok(());
            }

            // service is not ready yet, try 1 sec later
            actix_rt::time::sleep(Duration::from_millis(1000)).await;
        }
    }

    async fn service_request_status(&self, endpoint: &str) -> Option<reqwest::StatusCode> {
        let endpoint = format!("{}{}", self.address, endpoint);
        match self.service_request(&endpoint).await {
            Ok(response) => Some(response.status()),
            _ => None,
        }
    }

    async fn service_request(&self, endpoint: &str) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::Client::new().get(endpoint).send().await
    }
}

pub fn is_service_process_running() -> bool {
    let mut service_process = SERVICE_PROCESS.lock().unwrap();
    if service_process.is_none() {
        return true;
    }
    (*service_process).as_mut().unwrap().try_wait().is_ok()
}
