use std::env;
use std::sync::Arc;
use std::time::{Instant};

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use log::{info};
use reqwest::{Error, Response};
use rusoto_core::Region;
use rusoto_s3::{S3Client};
use ssz_rs::prelude::*;

use crate::models::StatusMessage;

mod models;
mod logging;
mod route_get_proof;
mod cache_sync_engine;
mod state;

async fn index() -> impl Responder {
    HttpResponse::Ok().json(StatusMessage { message: String::from("Welcome to our API!") })
}

async fn fetch_state(state_id: &str) -> Option<Vec<u8>> {
    let response = fetch_state_from_node(state_id).await;

    match response {
        Ok(res) if res.status().is_success() => {
            match res.bytes().await {
                Ok(bytes) => {
                    info!("Got state from lodestar");
                    Some(bytes.to_vec())
                }
                Err(e) => {
                    info!("Error reading bytes: {}", e);
                    None
                }
            }
        }
        _ => {
            info!("Error fetching state from lodestar");
            None
        }
    }
}

async fn fetch_state_from_node(state_id: &str) -> Result<Response, Error> {
    let client_http = reqwest::Client::new();
    info!("Attempting to fetch state {} from lodestar", state_id);
    let url = format!("http://lodestar:9596/eth/v2/debug/beacon/states/{}", state_id);

    let start = Instant::now();
    let response = client_http.get(&url)
        .header("Accept", "application/octet-stream")
        .send()
        .await;

    let duration = start.elapsed();
    info!("Successfully fetched state from node for {} in {:.2?} seconds.", state_id, duration.as_secs_f64());

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting up");
    logging::setup_logging().expect("failed to initialize logging");

    let s3_client = S3Client::new(Region::default());
    let s3_data = Arc::new(s3_client);

    let _background = tokio::spawn(async {
        cache_sync_engine::pre_cache_proofs().await;
    });

    let port = env::var("PORT").unwrap_or_else(|_| "9000".to_string());

    let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "local".to_string());
    let bind_address = match run_env.as_str() {
        "docker" => format!("0.0.0.0:{}", port),
        _ => format!("127.0.0.1:{}", port),
    };
    info!("Binding to {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(s3_data.clone()))
            .route("/", web::get().to(index))
            .route("/proof/state", web::get().to(route_get_proof::get_proof))
    })
        .bind(bind_address)?
        .run()
        .await
}