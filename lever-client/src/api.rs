use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use lever_core::Feature;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LoadFeatureResponse {
    results: Vec<Feature>,
}

pub async fn load_features() -> Option<Vec<Feature>> {
    let response = Request::get("http://localhost:8080/features")
        .send()
        .await
        .unwrap();

    let json: Result<LoadFeatureResponse, gloo_net::Error> = response.json().await;

    match json {
        Ok(feature) => Some(feature.results),
        Err(_) => None,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CreateFeatureRequest {
    name: String,
    enabled: bool,
}

pub async fn create_feature(name: String, enabled: bool) -> Option<Feature> {
    let create_feature_body = CreateFeatureRequest {
        name, enabled
    };

    let response: Result<Feature, gloo_net::Error> = Request::post("http://localhost:8080/features")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&create_feature_body).unwrap())
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await;

    match response {
        Ok(feature) => Some(feature),
        Err(_) => None,
    }
}