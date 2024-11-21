use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

pub mod base64 {
  use base64::prelude::*;
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  /// serialize bytes as base64
  pub fn serialize<S>(data: &[u8], s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    String::serialize(&BASE64_STANDARD.encode(data), s)
  }

  /// deserialize base64 to bytes
  pub fn deserialize<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(d)?;
    BASE64_STANDARD
      .decode(s.as_bytes())
      .map_err(serde::de::Error::custom)
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChunkTaskDetail {
  pub block_hashes: Vec<B256>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BundleTaskDetail {
  pub batch_proofs: Vec<BatchProof>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubCircuitRowUsage {
  pub name: String,
  pub row_number: usize,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChunkProof {
  #[serde(with = "base64")]
  pub protocol: Vec<u8>,
  #[serde(flatten)]
  pub proof: Proof,
  pub chunk_info: ChunkInfo,
  #[serde(default)]
  pub row_usages: Vec<SubCircuitRowUsage>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BatchProof {
  #[serde(with = "base64")]
  pub protocol: Vec<u8>,
  #[serde(flatten)]
  proof: Proof,
  pub batch_hash: B256,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BundleProof {
  #[serde(flatten)]
  on_chain_proof: Proof,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Proof {
  #[serde(with = "base64")]
  proof: Vec<u8>,
  #[serde(with = "base64")]
  instances: Vec<u8>,
  #[serde(with = "base64")]
  vk: Vec<u8>,
  pub git_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChunkInfo {
  /// Chain identifier
  pub chain_id: u64,
  /// state root before this chunk
  pub prev_state_root: B256,
  /// state root after this chunk
  pub post_state_root: B256,
  /// the withdraw root after this chunk
  pub withdraw_root: B256,
  /// the data hash of this chunk
  pub data_hash: B256,
  /// Flattened L2 tx bytes (RLP-signed) in this chunk.
  #[serde(with = "base64")]
  pub tx_bytes: Vec<u8>,
  /// if the chunk is a padded chunk
  pub is_padding: bool,
}
