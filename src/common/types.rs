pub type DbPool = sqlx::SqlitePool;
pub type DbExecutor = sqlx::SqliteConnection;
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}
