use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use std::{error, net::SocketAddr};
use tokio_postgres::{types::ToSql, NoTls};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres", NoTls)?;
    let pool = Pool::builder().build(manager).await?;

    let app = Router::new()
        .route("/1", post(insert_one_by_one_loop))
        //.route("/2", post(insert_multirow_once))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

// TODO: figure out why this doesn't work
#[debug_handler]
async fn insert_multirow_once(DbConn(conn): DbConn) -> Result<impl IntoResponse, StatusCode> {
    let statement = "INSERT INTO foo (a, b) VALUES ($1, $2), ($3, $4)";

    let mut params = Vec::<Box<dyn ToSql + Sync>>::with_capacity(6);

    for i in 1..=2 {
        params.push(Box::new(i));
        params.push(Box::new("s".to_owned()));
    }

    let params = params.iter().map(|x| x.as_ref()).collect::<Vec<_>>();

    conn.execute(statement, &params)
        .await
        .map_err(internal_error)?;
    Ok(StatusCode::OK)
}

/// Inserts records one at a time.
async fn insert_one_by_one_loop(DbConn(conn): DbConn) -> Result<impl IntoResponse, StatusCode> {
    for i in 1..=2 {
        conn.execute("INSERT INTO foo (a, b) VALUES ($1, $2)", &[&i, &"s"])
            .await
            .map_err(internal_error)?;
    }
    Ok(StatusCode::OK)
}

type ConnPool = Pool<PostgresConnectionManager<NoTls>>;

struct DbConn(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

#[async_trait]
impl<S> FromRequestParts<S> for DbConn
where
    ConnPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = ConnPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

fn internal_error<E>(_: E) -> StatusCode
where
    E: error::Error,
{
    StatusCode::INTERNAL_SERVER_ERROR
}
