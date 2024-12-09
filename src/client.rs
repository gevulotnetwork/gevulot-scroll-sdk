use async_trait::async_trait;

use crate::{
  GetVkRequest, GetVkResponse, ProveRequest, ProveResponse, QueryTaskRequest, QueryTaskResponse,
};

pub const DEFAULT_PROVING_SERVICE_URL: &str = "http://scroll-proving-service.gevulot.com:3000";

pub struct Client {
  proving_service_url: String,
  auth_token: String,
}

impl Client {
  pub fn new(proving_service_url: String, auth_token: Option<String>) -> Self {
    // NOTE: We always want to store the basic URL without trailing slash.
    let proving_service_url = proving_service_url.trim_end_matches('/').to_string();
    Client {
      proving_service_url,
      auth_token: auth_token.unwrap_or("".to_string()),
    }
  }

  pub async fn is_healthy(&self) -> bool {
    let client = reqwest::Client::new();
    let endpoint = self.proving_service_url.clone() + "/health";
    let response = client.get(endpoint).send().await.unwrap();

    response.text().await.unwrap() == "OK"
  }
}

#[async_trait]
impl ProvingService for Client {
  fn is_local(&self) -> bool {
    // NOTE: Saying this is local service is a dirty way to prevent running
    // Scroll Proving SDK with multiple workers configured - which makes
    // no sense here.
    true
  }

  async fn get_vk(&self, req: GetVkRequest) -> GetVkResponse {
    // Construct the URL, including query parameters.
    let client = reqwest::Client::new();
    let endpoint = self.proving_service_url.clone() + "/get_vk";
    let response = client
      .get(endpoint)
      .query(&[
        ("circuit_type", req.circuit_type.to_string()),
        ("circuit_version", req.circuit_version),
      ])
      .send()
      .await
      .unwrap();

    let vk_response: GetVkResponse = response.json().await.unwrap();
    vk_response
  }

  async fn prove(&self, req: ProveRequest) -> ProveResponse {
    // Construct the URL, including query parameters.
    let client = reqwest::Client::new();
    let endpoint = self.proving_service_url.clone() + "/prove_task";
    let response = client
      .post(endpoint)
      .json(&req)
      .bearer_auth(&self.auth_token) // NOTE: Auth token is required to submit a new task.
      .send()
      .await
      .unwrap();

    let prove_response: ProveResponse = response.json().await.unwrap();
    prove_response
  }

  async fn query_task(&self, req: QueryTaskRequest) -> QueryTaskResponse {
    // Construct the URL, including query parameters.
    let client = reqwest::Client::new();
    let endpoint = self.proving_service_url.clone() + "/query_task";
    let response = client
      .get(endpoint)
      .query(&[("task_id", req.task_id)])
      .send()
      .await
      .unwrap();

    let query_task_response: QueryTaskResponse = response.json().await.unwrap();
    query_task_response
  }
}

// Interface from the Scroll Proving SDK.
#[async_trait]
pub trait ProvingService {
  fn is_local(&self) -> bool;
  async fn get_vk(&self, req: GetVkRequest) -> GetVkResponse;
  async fn prove(&self, req: ProveRequest) -> ProveResponse;
  async fn query_task(&self, req: QueryTaskRequest) -> QueryTaskResponse;
}
