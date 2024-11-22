use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Supported circuit versions that SHOULD be supported by proving service.
pub const CIRCUIT_VERSION_0_13: &str = "v0.13.1";

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "prover")]
pub mod prover;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetVkRequest {
  pub circuit_type: CircuitType,
  pub circuit_version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetVkResponse {
  pub vk: String,
  pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProveRequest {
  pub circuit_type: CircuitType,
  pub circuit_version: String,
  pub hard_fork_name: String,
  pub input: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProveResponse {
  pub task_id: String,
  pub circuit_type: CircuitType,
  pub circuit_version: String,
  pub hard_fork_name: String,
  pub status: TaskStatus,
  pub created_at: f64,
  pub started_at: Option<f64>,
  pub finished_at: Option<f64>,
  pub compute_time_sec: Option<f64>,
  pub input: Option<String>,
  pub proof: Option<String>,
  pub vk: Option<String>,
  pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryTaskRequest {
  pub task_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryTaskResponse {
  pub task_id: String,
  pub circuit_type: CircuitType,
  pub circuit_version: String,
  pub hard_fork_name: String,
  pub status: TaskStatus,
  pub created_at: f64,
  pub started_at: Option<f64>,
  pub finished_at: Option<f64>,
  pub compute_time_sec: Option<f64>,
  pub input: Option<String>,
  pub proof: Option<String>,
  pub vk: Option<String>,
  pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum TaskStatus {
  Queued,
  Proving,
  Success,
  Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CircuitType {
  Undefined,
  Chunk,
  Batch,
  Bundle,
}

impl CircuitType {
  pub fn from_u8(v: u8) -> Self {
    match v {
      1 => CircuitType::Chunk,
      2 => CircuitType::Batch,
      3 => CircuitType::Bundle,
      _ => CircuitType::Undefined,
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      CircuitType::Undefined => 0,
      CircuitType::Chunk => 1,
      CircuitType::Batch => 2,
      CircuitType::Bundle => 3,
    }
  }

  pub fn from_string(value: &str) -> Self {
    match value {
      "chunk" => Self::Chunk,
      "bundle" => Self::Bundle,
      "batch" => Self::Batch,
      _ => Self::Undefined,
    }
  }
}

impl fmt::Display for CircuitType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Chunk => write!(f, "chunk"),
      Self::Batch => write!(f, "batch"),
      Self::Bundle => write!(f, "bundle"),
      Self::Undefined => write!(f, "undefined"),
    }
  }
}

impl Serialize for CircuitType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match *self {
      CircuitType::Undefined => serializer.serialize_u8(0),
      CircuitType::Chunk => serializer.serialize_u8(1),
      CircuitType::Batch => serializer.serialize_u8(2),
      CircuitType::Bundle => serializer.serialize_u8(3),
    }
  }
}

impl<'de> Deserialize<'de> for CircuitType {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let v: u8 = u8::deserialize(deserializer)?;
    Ok(CircuitType::from_u8(v))
  }
}
