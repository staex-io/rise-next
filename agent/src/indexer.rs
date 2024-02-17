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
    types::{Address, Block, Filter, Log, H256, U256},
    utils::{self, format_ether, keccak256, parse_ether},
};
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use tokio::{sync::Mutex, time::sleep};

use crate::{Config, Error};

pub(crate) async fn run(cfg: Config, dsn: String, port: u16, from_block: u64) -> Result<(), Error> {
    let database = Database::new(dsn).await?;

    let indexer = Indexer::new(database.clone()).await?;
    tokio::spawn(async move {
        if let Err(e) = indexer.run(cfg, from_block).await {
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
            ground_cycle_landing_hash: H256::from(keccak256(
                "Landing(uint256,address,address,address)".as_bytes(),
            )),
            ground_cycle_takeoff_hash: H256::from(keccak256(
                "Takeoff(uint256,address,address,address)".as_bytes(),
            )),
        })
    }

    async fn run(&self, cfg: Config, from_block: u64) -> Result<(), Error> {
        let did_contract_addr: Address = cfg.did_contract_addr.parse()?;
        let agreement_contract_addr: Address = cfg.agreement_contract_addr.parse()?;
        let ground_cycle_contract_addr: Address = cfg.ground_cycle_contract_addr.parse()?;
        let contracts: Vec<Address> = vec![
            did_contract_addr,
            agreement_contract_addr,
            ground_cycle_contract_addr,
        ];

        let provider: Provider<Http> = Provider::<Http>::try_from(cfg.rpc_url.clone())?;

        let mut from_block: u64 = from_block;
        let mut block_step: u64 = 2625;
        loop {
            let to_block: u64 = from_block + block_step;
            let block = provider.get_block(to_block).await?;
            trace!("scanning from {from_block} to {to_block} block");
            if block.is_none() {
                block_step /= 10;
                if block_step == 0 {
                    block_step = 1;
                }
                trace!("{} (to_block) is not exists yet, waiting", to_block);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            let block = block.unwrap();
            for contract in &contracts {
                let logs = provider
                    .get_logs(
                        &Filter::new().address(*contract).from_block(from_block).to_block(to_block),
                    )
                    .await?;
                self.process_logs(&block, logs).await?;
            }
            from_block += block_step;
            sleep(Duration::from_secs(4)).await;
        }
    }

    async fn process_logs(&self, block: &Block<H256>, logs: Vec<Log>) -> Result<(), Error> {
        for log in logs {
            let topic0 = log.topics[0];
            match topic0 {
                _ if (topic0 == self.did_updated_hash) => {
                    debug!("received did event");
                    let event = DIDContractEvents::decode_log(&RawLog::from(log))?;
                    event.save(&self.database, block.timestamp.as_u32()).await?;
                }
                _ if (topic0 == self.agreement_created_hash
                    || topic0 == self.agreement_signed_hash) =>
                {
                    debug!("received agreement event");
                    let event = AgreementContractEvents::decode_log(&RawLog::from(log))?;
                    event.save(&self.database, block.timestamp.as_u32()).await?;
                }
                _ if (topic0 == self.ground_cycle_landing_hash
                    || topic0 == self.ground_cycle_takeoff_hash) =>
                {
                    debug!("received ground cycle event");
                    let event = GroundCycleContractEvents::decode_log(&RawLog::from(log))?;
                    event.save(&self.database, block.timestamp.as_u32()).await?;
                }
                _ => continue,
            }
        }
        Ok(())
    }
}

trait DatabaseSaver: Send + Sync {
    fn save(
        &self,
        database: &Database,
        timestamp: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl DatabaseSaver for DIDContractEvents {
    async fn save(&self, database: &Database, _: u32) -> Result<(), Error> {
        match self {
            DIDContractEvents::UpdatedFilter(event) => {
                database
                    .save_station(
                        &format!("{:?}", Address::from(event.p0)),
                        &event.location,
                        &utils::format_ether(event.price),
                    )
                    .await
            }
            DIDContractEvents::RemovedFilter(_) => unimplemented!(),
        }
    }
}

impl DatabaseSaver for AgreementContractEvents {
    async fn save(&self, database: &Database, _: u32) -> Result<(), Error> {
        match self {
            AgreementContractEvents::CreatedFilter(event) => {
                database
                    .save_agreement(
                        &format!("{:?}", Address::from(event.0)),
                        &format!("{:?}", Address::from(event.1)),
                        &utils::format_ether(event.2),
                        false,
                    )
                    .await
            }
            AgreementContractEvents::SignedFilter(event) => {
                database
                    .save_agreement(
                        &format!("{:?}", Address::from(event.0)),
                        &format!("{:?}", Address::from(event.1)),
                        "",
                        true,
                    )
                    .await
            }
        }
    }
}

impl DatabaseSaver for GroundCycleContractEvents {
    async fn save(&self, database: &Database, timestamp: u32) -> Result<(), Error> {
        match self {
            GroundCycleContractEvents::LandingFilter(event) => {
                database
                    .save_landing(
                        event.0.as_u32(),
                        &format!("{:?}", Address::from(event.1)),
                        &format!("{:?}", Address::from(event.2)),
                        &format!("{:?}", Address::from(event.3)),
                        false,
                        false,
                        timestamp,
                    )
                    .await
            }
            GroundCycleContractEvents::TakeoffFilter(event) => {
                database
                    .save_landing(
                        event.0.as_u32(),
                        &format!("{:?}", Address::from(event.1)),
                        &format!("{:?}", Address::from(event.2)),
                        &format!("{:?}", Address::from(event.3)),
                        true,
                        false,
                        timestamp,
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
    price: String,
}

#[derive(sqlx::FromRow, Debug)]
struct DatabaseAgreement {
    station: String,
    entity: String,
    amount: String,
    is_signed: bool,
}

#[derive(sqlx::FromRow)]
struct DatabaseLanding {
    id: u32,
    drone: String,
    station: String,
    landlord: String,
    is_taken_off: bool,
    is_rejected: bool,
    date: u32,
}

#[derive(sqlx::FromRow)]
struct DatabaseStats {
    landings: u32,
    amount: String,
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

    async fn save_station(&self, address: &str, location: &str, price: &str) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into stations (address, location, price)
                values (?1, ?2, ?3) on conflict do update
                set location = ?2, price = ?3
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
        amount: &str,
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

    async fn get_agreement(&self, station: &str, entity: &str) -> Result<DatabaseAgreement, Error> {
        let mut conn = self.conn.lock().await;
        let agreement: DatabaseAgreement = sqlx::query_as::<_, DatabaseAgreement>(
            "select * from agreements where station = ?1 and entity = ?2",
        )
        .bind(station)
        .bind(entity)
        .fetch_one(&mut *conn)
        .await?;
        Ok(agreement)
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
            let address = address.to_lowercase();
            query.push(" where station = ");
            query.push_bind(address.clone());
            query.push(" or entity = ");
            query.push_bind(address);
            return Ok(query);
        }
        query.push(" limit ").push_bind(limit).push(" offset ").push_bind(offset);
        Ok(query)
    }

    #[allow(clippy::too_many_arguments)]
    async fn save_landing(
        &self,
        id: u32,
        drone: &str,
        station: &str,
        landlord: &str,
        is_taken_off: bool,
        is_rejected: bool,
        timestamp: u32,
    ) -> Result<(), Error> {
        let station_drone_agreement = self.get_agreement(station, drone).await?;
        let station_landlord_agreement = self.get_agreement(station, landlord).await?;

        let drone_stats = self.get_stats(drone).await?;
        self.update_stats(drone, drone_stats.landings + 1, &U256::from(0).to_string()).await?;

        let station_stats = self.get_stats(station).await?;
        self.update_stats(
            station,
            station_stats.landings + 1,
            &format_ether(
                parse_ether(station_stats.amount)?
                    .checked_add(parse_ether(&station_drone_agreement.amount)?)
                    .ok_or_else(|| "failed to add station stats amount".to_string())?
                    .checked_sub(parse_ether(&station_landlord_agreement.amount)?)
                    .unwrap_or(U256::from(0)),
            ),
        )
        .await?;

        let landlord_stats = self.get_stats(landlord).await?;
        self.update_stats(
            landlord,
            landlord_stats.landings + 1,
            &format_ether(
                parse_ether(landlord_stats.amount)?
                    .checked_add(parse_ether(&station_landlord_agreement.amount)?)
                    .ok_or_else(|| "failed to add landlord stats amount".to_string())?,
            ),
        )
        .await?;

        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into landings (id, drone, station, landlord, is_taken_off, is_rejected, date)
                values (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                on conflict do update set is_taken_off = ?5, is_rejected = ?6
            "#,
        )
        .bind(id)
        .bind(drone)
        .bind(station)
        .bind(landlord)
        .bind(is_taken_off)
        .bind(is_rejected)
        .bind(timestamp)
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn query_landings(
        &self,
        id: Option<u32>,
        address: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<DatabaseLanding>, Error> {
        let mut query: QueryBuilder<sqlx::Sqlite> =
            Self::prepare_query_landings::<sqlx::Sqlite>(&id, &address, limit, offset)?;
        trace!("sql query landings: {}", query.sql());
        let query = query.build_query_as::<DatabaseLanding>();
        let mut conn = self.conn.lock().await;
        let landings = query.fetch_all(&mut *conn).await?;
        Ok(landings)
    }

    fn prepare_query_landings<'a, DB: sqlx::Database>(
        id: &'a Option<u32>,
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
        if let Some(id) = &id {
            query.push(" where id = ");
            query.push_bind(id);
            return Ok(query);
        }
        if let Some(address) = &address {
            let address = address.to_lowercase();
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

    async fn update_stats(&self, address: &str, landings: u32, amount: &str) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query("update stats set landings = ?2, amount = ?3 where address = ?1")
            .bind(address.to_lowercase())
            .bind(landings)
            .bind(amount)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }

    async fn get_stats(&self, address: &str) -> Result<DatabaseStats, Error> {
        eprintln!("...");
        let mut conn = self.conn.lock().await;
        let stats: DatabaseStats = sqlx::query_as::<_, DatabaseStats>(
            "insert into stats (address, landings, amount) values (?1, 0, '0') on conflict do nothing returning *",
        )
        .bind(address.to_lowercase())
        .fetch_one(&mut *conn)
        .await?;
        Ok(stats)
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
        .route("/stats", get(get_stats))
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
    id: Option<u32>,
    address: Option<String>,
    #[serde(default = "default_limit")]
    limit: u32,
    #[serde(default = "default_offset")]
    offset: u32,
}

fn default_limit() -> u32 {
    10
}
fn default_offset() -> u32 {
    0
}

#[derive(Serialize)]
struct StationResponse {
    address: String,
    location: String,
    price: String,
}

#[derive(Serialize)]
struct AgreementResponse {
    station: String,
    entity: String,
    amount: String,
    is_signed: bool,
}

impl From<DatabaseAgreement> for AgreementResponse {
    fn from(value: DatabaseAgreement) -> Self {
        Self {
            station: value.station,
            entity: value.entity,
            amount: value.amount,
            is_signed: value.is_signed,
        }
    }
}

#[derive(Serialize)]
struct StatsResponse {
    landings: u32,
    amount: String,
}

#[derive(Serialize)]
struct LandingResponse {
    id: u32,
    drone: String,
    station: String,
    landlord: String,
    is_taken_off: bool,
    is_rejected: bool,
    date: u32,
    agreements: Option<Vec<AgreementResponse>>,
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
            price: val.price,
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
        external.push(val.into())
    }
    Ok((StatusCode::OK, Json(external)))
}

async fn get_landings(
    Query(params): Query<QueryParams>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let internal =
        database.query_landings(params.id, params.address, params.limit, params.offset).await?;
    let mut external: Vec<LandingResponse> = Vec::with_capacity(internal.len());
    for val in &internal {
        external.push(LandingResponse {
            id: val.id,
            drone: val.drone.clone(),
            station: val.station.clone(),
            landlord: val.landlord.clone(),
            is_taken_off: val.is_taken_off,
            is_rejected: val.is_rejected,
            date: val.date,
            agreements: None,
        })
    }
    // When user provides ID it means it wants to get particular landing.
    if params.id.is_some() && internal.len() == 1 {
        let internal = &internal[0];
        let agreement_drone = database.get_agreement(&internal.station, &internal.drone).await?;
        let agreement_landlord =
            database.get_agreement(&internal.station, &internal.landlord).await?;
        external[0].agreements = Some(vec![agreement_drone.into(), agreement_landlord.into()])
    }
    Ok((StatusCode::OK, Json(external)))
}

async fn get_stats(
    Query(params): Query<QueryParams>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if params.address.is_none() {
        return Err("address is empty".into());
    }
    let address = params.address.unwrap();
    let internal = database.get_stats(&address).await?;
    Ok((
        StatusCode::OK,
        Json(StatsResponse {
            landings: internal.landings,
            amount: internal.amount,
        }),
    ))
}

async fn fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
