
// common utils amonst job doers and job creators

use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::nn::Network;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobInfo {
    pub name: String,
    pub individual: Network,
}
