use std::{fs::OpenOptions, future::Future, io::ErrorKind, sync::Arc, time::Duration};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use contracts_rs::{AgreementContractEvents, DIDContractEvents, GroundCycleContractEvents};
use ethers::{
    abi::RawLog,
    contract::EthLogDecode,
    providers::{Http, Middleware, Provider},
    types::{Address, Filter, Log, H256},
    utils::keccak256,
};
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use tokio::{sync::Mutex, time::sleep};

use crate::{Config, Error, BLOCK_STEP};

pub(crate) async fn run(cfg: Config, dsn: String, port: u16) -> Result<(), Error> {
    let database = Database::new(dsn).await?;

    let indexer = Indexer::new(database.clone()).await?;
    tokio::spawn(async move {
        if let Err(e) = indexer.run(cfg).await {
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

    did_updated_hash: H256,
    agreement_created_hash: H256,
    agreement_signed_hash: H256,
    ground_cycle_landing_hash: H256,
    ground_cycle_takeoff_hash: H256,
}

impl Indexer {
    async fn new(database: Database) -> Result<Self, Error> {
        Ok(Self {
            database,
            did_updated_hash: H256::from(keccak256("Updated(address,string,uint256)".as_bytes())),
            agreement_created_hash: H256::from(keccak256(
                "Created(address,address,uint256)".as_bytes(),
            )),
            agreement_signed_hash: H256::from(keccak256("Signed(address,address)".as_bytes())),
            ground_cycle_landing_hash: H256::from(keccak256("Signed(address,address)".as_bytes())),
            ground_cycle_takeoff_hash: H256::from(keccak256("Signed(address,address)".as_bytes())),
        })
    }

    async fn run(&self, cfg: Config) -> Result<(), Error> {
        let did_contract_addr: Address = cfg.did_contract_addr.parse()?;
        let agreement_contract_addr: Address = cfg.agreement_contract_addr.parse()?;
        let ground_cycle_contract_addr: Address = cfg.ground_cycle_contract_addr.parse()?;
        let contracts: Vec<Address> = vec![
            did_contract_addr,
            agreement_contract_addr,
            ground_cycle_contract_addr,
        ];

        let provider: Provider<Http> = Provider::<Http>::try_from(cfg.rpc_url.clone())?;

        let mut from_block: u64 = 0;
        loop {
            let to_block: u64 = from_block + BLOCK_STEP;
            if provider.get_block(to_block).await?.is_none() {
                trace!("{} (to_block) is not exists yet, waiting", to_block);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            for contract in &contracts {
                let logs = provider
                    .get_logs(
                        &Filter::new().address(*contract).from_block(from_block).to_block(to_block),
                    )
                    .await?;
                self.process_logs(logs).await?;
            }
            from_block += BLOCK_STEP;
        }
    }

    async fn process_logs(&self, logs: Vec<Log>) -> Result<(), Error> {
        for log in logs {
            let topic0 = log.topics[0];
            match topic0 {
                _ if (topic0 == self.did_updated_hash) => {
                    let event = DIDContractEvents::decode_log(&RawLog::from(log))?;
                    event.save(&self.database).await?;
                }
                _ if (topic0 == self.agreement_created_hash
                    || topic0 == self.agreement_signed_hash) =>
                {
                    let event = AgreementContractEvents::decode_log(&RawLog::from(log))?;
                    event.save(&self.database).await?;
                }
                _ if (topic0 == self.ground_cycle_landing_hash
                    || topic0 == self.ground_cycle_takeoff_hash) =>
                {
                    let event = GroundCycleContractEvents::decode_log(&RawLog::from(log))?;
                    event.save(&self.database).await?;
                }
                _ => continue,
            }
        }
        Ok(())
    }
}

trait DatabaseSaver: Send + Sync {
    fn save(&self, database: &Database) -> impl Future<Output = Result<(), Error>> + Send;
}

impl DatabaseSaver for DIDContractEvents {
    async fn save(&self, database: &Database) -> Result<(), Error> {
        match self {
            DIDContractEvents::UpdatedFilter(event) => {
                database
                    .save_station(
                        &format!("{:?}", Address::from(event.p0)),
                        &event.location,
                        (event.price / ethers::utils::WEI_IN_ETHER).as_u32() as i64,
                    )
                    .await
            }
            DIDContractEvents::RemovedFilter(_) => unimplemented!(),
        }
    }
}

impl DatabaseSaver for AgreementContractEvents {
    async fn save(&self, database: &Database) -> Result<(), Error> {
        match self {
            AgreementContractEvents::CreatedFilter(event) => {
                database
                    .save_agreement(
                        &format!("{:?}", Address::from(event.0)),
                        &format!("{:?}", Address::from(event.1)),
                        (event.2 / ethers::utils::WEI_IN_ETHER).as_u32() as i64,
                        false,
                    )
                    .await
            }
            AgreementContractEvents::SignedFilter(event) => {
                database
                    .save_agreement(
                        &format!("{:?}", Address::from(event.0)),
                        &format!("{:?}", Address::from(event.1)),
                        0,
                        true,
                    )
                    .await
            }
        }
    }
}

impl DatabaseSaver for GroundCycleContractEvents {
    async fn save(&self, database: &Database) -> Result<(), Error> {
        match self {
            GroundCycleContractEvents::LandingFilter(event) => {
                database
                    .save_landing(
                        event.0.as_u64() as i64,
                        &format!("{:?}", Address::from(event.1)),
                        &format!("{:?}", Address::from(event.2)),
                        &format!("{:?}", Address::from(event.3)),
                        false,
                        false,
                    )
                    .await
            }
            GroundCycleContractEvents::TakeoffFilter(event) => {
                database
                    .save_landing(
                        event.0.as_u64() as i64,
                        &format!("{:?}", Address::from(event.1)),
                        &format!("{:?}", Address::from(event.2)),
                        &format!("{:?}", Address::from(event.3)),
                        true,
                        false,
                    )
                    .await
            }
            _ => unimplemented!(),
        }
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

    async fn save_station(&self, address: &str, location: &str, price: i64) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into stations (address, location, price)
                values (?1, ?2, ?3)
            "#,
        )
        .bind(address)
        .bind(location)
        .bind(price)
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
        &self,
        station: &str,
        entity: &str,
        amount: i64,
        is_signed: bool,
    ) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into agreements (station, entity, amount, is_signed)
                values (?1, ?2, ?3, ?4)
                on conflict do update set is_signed = ?4
            "#,
        )
        .bind(station)
        .bind(entity)
        .bind(amount)
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
        &self,
        id: i64,
        drone: &str,
        station: &str,
        landlord: &str,
        is_taken_off: bool,
        is_rejected: bool,
    ) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into landings (id, drone, station, landlord, is_taken_off, is_rejected)
                values (?1, ?2, ?3, ?4, ?5, ?6)
                on conflict do update set is_taken_off = ?5, is_rejected = ?6
            "#,
        )
        .bind(id)
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
    for val in internal {
        external.push(StationResponse {
            address: val.address.clone(),
            location: val.location.clone(),
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
    for val in internal {
        external.push(AgreementResponse {
            station: val.station.clone(),
            entity: val.entity.clone(),
            is_signed: val.is_signed,
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
    for val in internal {
        external.push(LandingResponse {
            id: val.id,
            drone: val.drone.clone(),
            station: val.station.clone(),
            landlord: val.landlord.clone(),
            is_taken_off: val.is_taken_off,
            is_rejected: val.is_rejected,
        })
    }
    Ok((StatusCode::OK, Json(external)))
}

async fn fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
