use std::time::Instant;

use actix_web::{HttpResponse, web};
use log::info;
use ssz_rs::prelude::*;

use crate::models::{QueryInfo};
use crate::state::get_and_cache_proof;

pub async fn get_proof(query: web::Query<QueryInfo>) -> Result<HttpResponse, actix_web::Error> {
    info!("API Hit");
    let start = Instant::now();

    let path = &query.path;
    let state_id = &query.state_id;

    let proof_json = get_and_cache_proof(state_id, path).await;

    let duration = start.elapsed();
    info!("Time taken to get proof: {:?}", duration);
    Ok(HttpResponse::Ok().content_type("application/json").body(proof_json))
}

