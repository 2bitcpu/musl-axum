pub mod content;

use crate::common::{
    config::{CORS_ORIGINS, SERVE_DIR},
    types::DbPool,
};
use crate::handler::content as content_handler;
use crate::use_case::Module;

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::get_service,
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn create_handler(pool: DbPool) -> Router {
    let module = Arc::new(Module::new(pool));

    let content_router = Router::new()
        .route("/create", axum::routing::post(content_handler::create))
        .route("/find/{id}", axum::routing::get(content_handler::find))
        .route("/edit", axum::routing::post(content_handler::edit))
        .route("/remove/{id}", axum::routing::get(content_handler::remove))
        .with_state(module);

    let api = Router::new().nest("/content", content_router);

    let api = match &*CORS_ORIGINS {
        Some(origins) => match origins.len() {
            0 => api,
            _ => {
                let cors = CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_origin(
                        origins
                            .iter()
                            .map(|s| s.parse::<HeaderValue>().unwrap())
                            .collect::<Vec<_>>(),
                    );
                api.layer(cors)
            }
        },
        None => api,
    };

    match &*SERVE_DIR {
        Some(dir) => Router::new()
            .nest("/service", api)
            .fallback(get_service(ServeDir::new(dir))),
        None => Router::new().nest("/service", api),
    }
}
