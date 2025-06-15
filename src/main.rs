use web_api::common::{
    config::{DB_URL, HOST_NAME},
    setup,
    types::{BoxError, DbPool},
};
use web_api::handler::create_handler;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let pool: DbPool = setup::init_db(&*DB_URL).await?;

    let api = create_handler(pool);

    let listener = tokio::net::TcpListener::bind(&*HOST_NAME).await?;

    axum::serve(listener, api.into_make_service())
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await?;

    Ok(())
}
