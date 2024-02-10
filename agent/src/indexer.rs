use std::{fs::OpenOptions, io::ErrorKind, sync::Arc};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use tokio::sync::Mutex;

use crate::Error;

pub(crate) async fn run(dsn: String, port: u16) -> Result<(), Error> {
    let database = Database::new(dsn).await?;

    let mut indexer = Indexer::new(database.clone()).await?;
    tokio::spawn(async move {
        if let Err(e) = indexer.run().await {
            error!("failed to run indexer: {e}");
        }
    });

    tokio::spawn(async move {
        if let Err(e) = run_api(port, database).await {
            error!("failed to run api: {e}")
        }
    });

    Ok(())
}

struct Indexer {
    database: Database,
}

impl Indexer {
    async fn new(database: Database) -> Result<Self, Error> {
        Ok(Self { database })
    }

    async fn run(&mut self) -> Result<(), Error> {
        todo!()
    }

    async fn process_event(&mut self) -> Result<(), Error> {
        todo!()
    }
}

#[derive(sqlx::FromRow)]
struct DatabaseStation {
    address: String,
    location: String,
}

#[derive(sqlx::FromRow)]
struct DatabaseAgreement {
    station: String,
    entity: String,
    is_signed: bool,
}

#[derive(sqlx::FromRow)]
struct DatabaseLanding {
    id: i64,
    drone: String,
    station: String,
    landlord: String,
    is_taken_off: bool,
    is_rejected: bool,
}

#[derive(Clone)]
struct Database {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    async fn new(dsn: String) -> Result<Self, Error> {
        // Create file if not exists to be able to open and migrate.
        let file_name = dsn.split(':').collect::<Vec<&str>>()[1];
        if let Err(e) =
            OpenOptions::new().read(true).write(true).create(true).create_new(true).open(file_name)
        {
            match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => return Err(e.into()),
            }
        }

        let mut conn = SqliteConnection::connect(&dsn).await?;
        conn.ping().await?;

        let migrator = sqlx::migrate!("./migrations/");
        migrator.run_direct(&mut conn).await?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    async fn save_station(&mut self, address: &str, location: &str) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into stations (address, location)
                values (?1, ?2)
            "#,
        )
        .bind(address)
        .bind(location)
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn query_stations(&self, limit: u32, offset: u32) -> Result<Vec<DatabaseStation>, Error> {
        let mut conn = self.conn.lock().await;
        let stations: Vec<DatabaseStation> =
            sqlx::query_as::<_, DatabaseStation>("select * from stations limit ?1 offset ?2")
                .bind(limit)
                .bind(offset)
                .fetch_all(&mut *conn)
                .await?;
        Ok(stations)
    }

    async fn save_agreement(
        &mut self,
        station: &str,
        entity: &str,
        is_signed: bool,
    ) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into agreements (station, entity, is_signed)
                values (?1, ?2, ?3)
                on conflict do update set is_signed = ?3
            "#,
        )
        .bind(station)
        .bind(entity)
        .bind(is_signed)
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn query_agreements(
        &self,
        address: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<DatabaseAgreement>, Error> {
        let mut query: QueryBuilder<sqlx::Sqlite> =
            Self::prepare_query_agreements::<sqlx::Sqlite>(&address, limit, offset)?;
        trace!("sql query agreements: {}", query.sql());
        let query = query.build_query_as::<DatabaseAgreement>();
        let mut conn = self.conn.lock().await;
        let agreements = query.fetch_all(&mut *conn).await?;
        Ok(agreements)
    }

    fn prepare_query_agreements<'a, DB: sqlx::Database>(
        address: &'a Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<QueryBuilder<'a, DB>, Error>
    where
        std::string::String: sqlx::Encode<'a, DB>,
        std::string::String: sqlx::Type<DB>,
        u32: sqlx::Encode<'a, DB>,
        u32: sqlx::Type<DB>,
    {
        let mut query: QueryBuilder<DB> = QueryBuilder::new("select * from agreements");
        if let Some(address) = &address {
            query.push(" where station = ");
            query.push_bind(address.clone());
            query.push(" or entity = ");
            query.push_bind(address);
            return Ok(query);
        }
        query.push(" limit ").push_bind(limit).push(" offset ").push_bind(offset);
        Ok(query)
    }

    async fn save_landing(
        &mut self,
        drone: &str,
        station: &str,
        landlord: &str,
        is_taken_off: bool,
        is_rejected: bool,
    ) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into landings (drone, station, landlord, is_taken_off, is_rejected)
                values (?1, ?2, ?3, ?4, ?5)
                on conflict do update set is_taken_off = ?4, is_rejected = ?5
            "#,
        )
        .bind(drone)
        .bind(station)
        .bind(landlord)
        .bind(is_taken_off)
        .bind(is_rejected)
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn query_landings(
        &self,
        address: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<DatabaseLanding>, Error> {
        let mut query: QueryBuilder<sqlx::Sqlite> =
            Self::prepare_query_landings::<sqlx::Sqlite>(&address, limit, offset)?;
        trace!("sql query landings: {}", query.sql());
        let query = query.build_query_as::<DatabaseLanding>();
        let mut conn = self.conn.lock().await;
        let landings = query.fetch_all(&mut *conn).await?;
        Ok(landings)
    }

    fn prepare_query_landings<'a, DB: sqlx::Database>(
        address: &'a Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<QueryBuilder<'a, DB>, Error>
    where
        std::string::String: sqlx::Encode<'a, DB>,
        std::string::String: sqlx::Type<DB>,
        u32: sqlx::Encode<'a, DB>,
        u32: sqlx::Type<DB>,
    {
        let mut query: QueryBuilder<DB> = QueryBuilder::new("select * from landings");
        if let Some(address) = &address {
            query.push(" where drone = ");
            query.push_bind(address.clone());
            query.push(" or station = ");
            query.push_bind(address.clone());
            query.push(" or landlord = ");
            query.push_bind(address);
            return Ok(query);
        }
        query.push(" limit ").push_bind(limit).push(" offset ").push_bind(offset);
        Ok(query)
    }
}

struct ErrorResponse {
    status_code: StatusCode,
    message: String,
}

impl<T: ToString> From<T> for ErrorResponse {
    fn from(value: T) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        if self.status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error!("internal server error: {}", self.message);
        }
        if self.message.is_empty() {
            self.status_code.into_response()
        } else {
            (self.status_code, self.message).into_response()
        }
    }
}

async fn run_api(port: u16, database: Database) -> Result<(), Error> {
    let app = Router::new()
        .route("/stations", get(get_stations))
        .route("/agreements", get(get_agreements))
        .route("/landings", get(get_landings))
        .layer(Extension(database))
        .fallback(fallback);
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listen on {addr} for HTTP requests");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
struct QueryParams {
    address: Option<String>,
    #[serde(default)]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            address: None,
            limit: 10,
            offset: 0,
        }
    }
}

#[derive(Serialize)]
struct StationResponse {
    address: String,
    location: String,
}

#[derive(Serialize)]
struct AgreementResponse {
    station: String,
    entity: String,
    is_signed: bool,
}

#[derive(Serialize)]
struct LandingResponse {
    id: i64,
    drone: String,
    station: String,
    landlord: String,
    is_taken_off: bool,
    is_rejected: bool,
}

async fn get_stations(
    Query(params): Query<QueryParams>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let internal = database.query_stations(params.limit, params.offset).await?;
    let mut external: Vec<StationResponse> = Vec::with_capacity(internal.len());
    for i in 0..internal.len() {
        external.push(StationResponse {
            address: internal[i].address.clone(),
            location: internal[i].location.clone(),
        })
    }
    Ok((StatusCode::OK, Json(external)))
}

async fn get_agreements(
    Query(params): Query<QueryParams>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let internal = database.query_agreements(params.address, params.limit, params.offset).await?;
    let mut external: Vec<AgreementResponse> = Vec::with_capacity(internal.len());
    for i in 0..internal.len() {
        external.push(AgreementResponse {
            station: internal[i].station.clone(),
            entity: internal[i].entity.clone(),
            is_signed: internal[i].is_signed,
        })
    }
    Ok((StatusCode::OK, Json(external)))
}

async fn get_landings(
    Query(params): Query<QueryParams>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let internal = database.query_landings(params.address, params.limit, params.offset).await?;
    let mut external: Vec<LandingResponse> = Vec::with_capacity(internal.len());
    for i in 0..internal.len() {
        external.push(LandingResponse {
            id: internal[i].id,
            drone: internal[i].drone.clone(),
            station: internal[i].station.clone(),
            landlord: internal[i].landlord.clone(),
            is_taken_off: internal[i].is_taken_off,
            is_rejected: internal[i].is_rejected,
        })
    }
    Ok((StatusCode::OK, Json(external)))
}

async fn fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
