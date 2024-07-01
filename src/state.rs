use ethereum_consensus::deneb::mainnet;
use ethereum_consensus::ssz::prelude::{PathElement, Prove};
use log::info;
use redis::Client;
use crate::models::SerializableProof;

pub async fn get_and_cache_proof(state_id: &String, path: &String) -> String {
    let path_elements: Vec<String> = path.split(',').map(String::from).collect();

    // TODO find out how to do proper resource management for things like db connections
    let client = Client::open("redis://redis-stack/").unwrap();
    let mut conn = client.get_connection().unwrap();

    let cache_key = format!("proof:{}:{}", state_id, path.replace(",", ":"));
    if state_id != "head" {
        match redis::cmd("GET").arg(&cache_key).query::<Option<String>>(&mut conn) {
            Ok(cached) => {
                if let Some(cached_proof) = cached {
                    info!("Got proof for key {}", cache_key);
                    return cached_proof;
                }
            }
            Err(..) => {}
        }
    }

    let bytes = crate::fetch_state(&state_id).await.unwrap();

    info!("Deserializing state");
    let mut data = <mainnet::BeaconState as ethereum_consensus::ssz::prelude::Deserialize>::deserialize(&bytes[..]).unwrap();
    info!("Deserializing completed");

    let path_elements: Vec<PathElement> = path_elements.iter().map(|s| PathElement::from(s.as_str())).collect();

    info!("Computing proof");
    let (proof, _) = data.prove(&path_elements).unwrap();
    info!("Proof computed");

    let serializable_proof = SerializableProof {
        leaf: format!("{:?}", proof.leaf),
        branch: proof.branch.iter().map(|b| format!("{:?}", b)).collect(),
        index: proof.index,
    };

    let proof_json = serde_json::to_string(&serializable_proof).unwrap();

    let ttl = 1209600 / 2;

    if state_id != "head" {
        let _: () = redis::cmd("SET")
            .arg(&cache_key)
            .arg(&proof_json)
            .arg("EX")
            .arg(ttl)
            .query(&mut conn)
            .unwrap();
        info!("Cached proof for key {}", cache_key)
    }
    proof_json
}
