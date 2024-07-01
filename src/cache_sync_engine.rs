use std::time::Duration;

use reqwest::Error;

use crate::models::FinalisedBeaconState;
use crate::state::get_and_cache_proof;

async fn get_latest_finalised_header() -> Result<FinalisedBeaconState, Error> {
    let client_http = reqwest::Client::new();
    let url = "http://lodestar:9596/eth/v1/beacon/light_client/finality_update".to_string();

    let response = client_http.get(&url)
        .send()
        .await?
        .json::<FinalisedBeaconState>()
        .await?;

    Ok(response)
}

pub async fn pre_cache_proofs() {
    loop {
        match get_latest_finalised_header().await {
            Ok(latest_header) => {
                let slot = latest_header.get_slot();
                println!("Current finalized slot ID: {:?}", slot);

                let paths = vec!["finalized_checkpoint,root"];

                for path in &paths {
                    println!("Pre-caching proof for path: {}, slot {}", path, slot);
                    get_and_cache_proof(&slot.to_string(), &path.to_string()).await;
                }
            }
            Err(e) => {
                println!("Error fetching latest finalized header: {:?}", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(12)).await;
    }
}