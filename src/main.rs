use axum::{
    BoxError, Router,
    extract::{FromRequest, Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, sqlite::SqliteConnectOptions};
use std::str::FromStr;

type DbPool = sqlx::SqlitePool;
type DbExecutor = sqlx::SqliteConnection;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Content {
    pub id: i64,
    pub publish_at: Option<DateTime<Utc>>,
    pub title: String,
    pub body: String,
}

#[derive(Clone)]
struct AppState {
    pub pool: DbPool,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let pool = init_db(":memory:").await?;
    let state = AppState { pool };

    let app = Router::new()
        .route("/create", post(create))
        .route("/find/{id}", get(find))
        .route("/edit", post(edit))
        .route("/remove/{id}", get(remove))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await?;

    Ok(())
}

//_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_//
async fn init_db(conn: &str) -> Result<DbPool, BoxError> {
    let options = SqliteConnectOptions::from_str(conn)?.create_if_missing(true);
    let pool = sqlx::SqlitePool::connect_with(options).await?;
    if let Ok(query) = tokio::fs::read_to_string("migrate.sql").await {
        sqlx::query(&query).execute(&pool).await?;
    }
    Ok(pool)
}

//_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_//
async fn create(
    State(state): State<AppState>,
    Json(entity): Json<Content>,
) -> Result<impl IntoResponse, ApiError> {
    let mut executor = state
        .pool
        .begin()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;

    let result = insert(&mut *executor, entity).await?;

    executor
        .commit()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;

    Ok(Json(result))
}

async fn insert(executor: &mut DbExecutor, entity: Content) -> Result<Content, BoxError> {
    let sql = "INSERT INTO content (publish_at,title,body) VALUES ($1,$2,$3) RETURNING *";
    Ok(sqlx::query_as::<_, Content>(sql)
        .bind(entity.publish_at)
        .bind(entity.title)
        .bind(entity.body)
        .fetch_one(&mut *executor)
        .await?)
}

//_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_//
async fn find(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let mut executor = state
        .pool
        .acquire()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;
    let result = select(&mut *executor, id).await?;

    Ok(Json(result))
}

async fn select(executor: &mut DbExecutor, id: i64) -> Result<Option<Content>, BoxError> {
    let sql = "SELECT * FROM content WHERE id=$1";
    Ok(sqlx::query_as::<_, Content>(sql)
        .bind(id)
        .fetch_optional(&mut *executor)
        .await?)
}

//_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_//
async fn edit(
    State(state): State<AppState>,
    Json(entity): Json<Content>,
) -> Result<impl IntoResponse, ApiError> {
    let mut executor = state
        .pool
        .begin()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;

    let result = update(&mut *executor, entity).await?;

    executor
        .commit()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;

    Ok(Json(result))
}

async fn update(executor: &mut DbExecutor, entity: Content) -> Result<Content, BoxError> {
    let sql = "UPDATE content SET publish_at=$2,title=$3,body=$4 WHERE id=$1 RETURNING *";
    Ok(sqlx::query_as::<_, Content>(sql)
        .bind(entity.id)
        .bind(entity.publish_at)
        .bind(entity.title)
        .bind(entity.body)
        .fetch_one(&mut *executor)
        .await?)
}

//_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_//
async fn remove(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let mut executor = state
        .pool
        .begin()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;

    let result = delete(&mut *executor, id).await?;

    executor
        .commit()
        .await
        .map_err(|e| ApiError::ProcessFailed(e.into()))?;

    Ok(Json(serde_json::json!({"rowsAffected":result})))
}

async fn delete(executor: &mut DbExecutor, id: i64) -> Result<u64, BoxError> {
    let sql = "DELETE FROM content WHERE id=$1 RETURNING *";
    Ok(sqlx::query(sql)
        .bind(id)
        .execute(&mut *executor)
        .await?
        .rows_affected())
}

//_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_//
#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiError))]
struct ApiResponse<T>(T);

impl<T> IntoResponse for ApiResponse<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    RequestTimeout,
    ProcessFailed(BoxError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request".to_owned()),
            ApiError::RequestTimeout => (StatusCode::REQUEST_TIMEOUT, "Request Timeout".to_owned()),
            ApiError::ProcessFailed(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error {err}").to_owned(),
            ),
        };

        (status, ApiResponse(ErrorResponse { message })).into_response()
    }
}

impl From<BoxError> for ApiError {
    fn from(error: BoxError) -> Self {
        Self::ProcessFailed(error)
    }
}
