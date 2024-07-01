use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
pub struct StatusMessage {
    pub message: String,
}

#[derive(Deserialize)]
pub struct QueryInfo {
    pub state_id: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct SerializableProof {
    pub leaf: String,
    pub branch: Vec<String>,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Head {
    pub slot: String,
    pub root: String,
    pub execution_optimistic: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Root {
    pub data: Vec<Head>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beacon {
    pub slot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalizedHeader {
    pub beacon: Beacon,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub finalized_header: FinalizedHeader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalisedBeaconState {
    pub data: Data,
}

impl FinalisedBeaconState {
    pub fn get_slot(&self) -> &str {
        &self.data.finalized_header.beacon.slot
    }
}