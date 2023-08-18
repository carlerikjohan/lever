use actix_web::{post, get, web, Responder, Result};
use lever_core::Feature;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::response::Response;

pub type Features = Response<Feature>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFeatureRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
}

impl CreateFeatureRequest {
    pub fn to_feature(&self) -> Option<Feature> {
        match (&self.name, &self.enabled) {
            (Some(name), Some(enabled)) => Some(Feature::new(name.to_string(), enabled.clone())),
            _ => None,
        }
    }
}

#[post("/features")]
async fn create(feature: web::Json<CreateFeatureRequest>) -> Result<impl Responder> {
    let feature = feature.to_feature();

    // MATCH SOMETHING HERE

    Ok(web::Json(feature))
}

#[get("/features")]
pub async fn list() -> Result<impl Responder> {
    let features = Features { results: vec![
        Feature {
            id: Uuid::new_v4(),
            name: "new_interview_kit".to_string(),
            enabled: true,
        },
        Feature {
            id: Uuid::new_v4(),
            name: "old_interview_kit".to_string(),
            enabled: false,
        },
        Feature {
            id: Uuid::new_v4(),
            name: "new_old_interview_kit".to_string(),
            enabled: true,
        },
        Feature {
            id: Uuid::new_v4(),
            name: "new_interview_kit_2.0".to_string(),
            enabled: true,
        },
        Feature {
            id: Uuid::new_v4(),
            name: "new_new_interview_kit_final".to_string(),
            enabled: true,
        }
    ] };

    Ok(web::Json(features))
}

#[get("/features/{name}")]
pub async fn get(path: web::Path<(String,)>) -> Result<impl Responder> {
    let name = path.into_inner().0;

    // TODO: find feature by id

    let feature: Feature = Feature::new(name, true);

    Ok(web::Json(feature))
}