use crate::middleware::error_handler::ApiError;
use crate::model::dto::content::ContentDto;
use crate::use_case::{Module, ModuleExtend};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn create(
    State(module): State<Arc<Module>>,
    Json(dto): Json<ContentDto>,
) -> Result<impl IntoResponse, ApiError> {
    let result = module.content().create(dto).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn find(
    State(module): State<Arc<Module>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let result = module.content().find(id).await?;
    Ok((StatusCode::OK, Json(result)))
}

pub async fn edit(
    State(module): State<Arc<Module>>,
    Json(dto): Json<ContentDto>,
) -> Result<impl IntoResponse, ApiError> {
    let result = module.content().edit(dto).await?;
    Ok((StatusCode::OK, Json(result)))
}

pub async fn remove(
    State(module): State<Arc<Module>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let result = module.content().remove(id).await?;
    Ok((StatusCode::OK, Json(result)))
}
