use std::{
    fs::OpenOptions, future::Future, io::ErrorKind, str::FromStr, sync::Arc, time::Duration,
};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use contracts_rs::{
    AgreementContractEvents, DIDContractEvents, GroundCycleContractEvents,
    GroundCycleNoCryptoContractEvents,
};
use ethers::{
    abi::RawLog,
    contract::EthLogDecode,
    providers::{Http, Middleware, Provider},
    types::{Address, Block, Filter, Log, H160, H256},
    utils::{format_ether, keccak256},
};
use log::{debug, error, info, trace};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use tokio::{sync::Mutex, time::sleep};

use crate::{Config, Error};

pub(crate) async fn run(
    cfg: Config,
    dsn: String,
    host: String,
    port: u16,
    from_block: u64,
) -> Result<(), Error> {
    let database = Database::new(dsn).await?;

    let indexer = Indexer::new(database.clone()).await?;
    tokio::spawn(async move {
        if let Err(e) = indexer.run(cfg, from_block).await {
            error!("failed to run indexer: {:?}", e);
        }
    });

    tokio::spawn(async move {
        if let Err(e) = run_api(host, port, database).await {
            error!("failed to run api: {:?}", e)
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
        let ground_cycle_no_crypto_contract_addr: Address =
            cfg.ground_cycle_no_crypto_contract_addr.parse()?;
        let contracts: Vec<Address> = vec![
            did_contract_addr,
            agreement_contract_addr,
            ground_cycle_contract_addr,
            ground_cycle_no_crypto_contract_addr,
        ];

        let provider: Provider<Http> = Provider::<Http>::try_from(cfg.rpc_url.clone())?;

        let latest_block = provider.get_block_number().await?;
        debug!("latest block number in selected network is {latest_block}");

        let mut from_block: u64 = from_block;
        let mut block_step: u64 = 50_000;
        loop {
            let to_block: u64 = from_block + block_step;
            let block = provider.get_block(to_block).await?;
            trace!("scanning from {from_block} to {to_block} block");
            if block.is_none() {
                block_step /= 2;
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
                self.process_logs(
                    &did_contract_addr,
                    &agreement_contract_addr,
                    &ground_cycle_contract_addr,
                    &ground_cycle_no_crypto_contract_addr,
                    &block,
                    logs,
                )
                .await?;
            }
            from_block += block_step;
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn process_logs(
        &self,
        did_contract_addr: &H160,
        agreement_contract_addr: &H160,
        ground_cycle_contract_addr: &H160,
        ground_cycle_no_crypto_contract_addr: &H160,
        block: &Block<H256>,
        logs: Vec<Log>,
    ) -> Result<(), Error> {
        for log in logs {
            let topic0 = log.topics[0];
            if &log.address == did_contract_addr {
                self.process_did_log(block, log, topic0).await?;
            } else if &log.address == agreement_contract_addr {
                self.process_agreement_log(block, log, topic0).await?;
            } else if &log.address == ground_cycle_contract_addr {
                self.process_ground_cycle_log(block, log, topic0).await?;
            } else if &log.address == ground_cycle_no_crypto_contract_addr {
                self.process_ground_cycle_no_crypto_log(block, log, topic0).await?;
            }
        }
        Ok(())
    }

    async fn process_did_log(
        &self,
        block: &Block<H256>,
        log: Log,
        topic0: H256,
    ) -> Result<(), Error> {
        if topic0 != self.did_updated_hash {
            return Ok(());
        }
        debug!("received did event");
        let event = DIDContractEvents::decode_log(&RawLog::from(log))?;
        event.save(&self.database, block.timestamp.as_u32()).await
    }

    async fn process_agreement_log(
        &self,
        block: &Block<H256>,
        log: Log,
        topic0: H256,
    ) -> Result<(), Error> {
        if topic0 != self.agreement_created_hash && topic0 != self.agreement_signed_hash {
            return Ok(());
        }
        debug!("received agreement event");
        let event = AgreementContractEvents::decode_log(&RawLog::from(log))?;
        event.save(&self.database, block.timestamp.as_u32()).await
    }

    async fn process_ground_cycle_log(
        &self,
        block: &Block<H256>,
        log: Log,
        topic0: H256,
    ) -> Result<(), Error> {
        if topic0 != self.ground_cycle_landing_hash && topic0 != self.ground_cycle_takeoff_hash {
            return Ok(());
        }
        debug!("received ground cycle event");
        let event = GroundCycleContractEvents::decode_log(&RawLog::from(log))?;
        event.save(&self.database, block.timestamp.as_u32()).await
    }

    async fn process_ground_cycle_no_crypto_log(
        &self,
        block: &Block<H256>,
        log: Log,
        topic0: H256,
    ) -> Result<(), Error> {
        if topic0 != self.ground_cycle_landing_hash && topic0 != self.ground_cycle_takeoff_hash {
            return Ok(());
        }
        debug!("received ground cycle no crypto event");
        let event = GroundCycleNoCryptoContractEvents::decode_log(&RawLog::from(log))?;
        event.save(&self.database, block.timestamp.as_u32()).await
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
                        &format_ether(event.price),
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
                        &format_ether(event.2),
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
                        false,
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
                        false,
                    )
                    .await
            }
            _ => unimplemented!(),
        }
    }
}

impl DatabaseSaver for GroundCycleNoCryptoContractEvents {
    async fn save(&self, database: &Database, timestamp: u32) -> Result<(), Error> {
        match self {
            GroundCycleNoCryptoContractEvents::LandingFilter(event) => {
                database
                    .save_landing(
                        event.0.as_u32(),
                        &format!("{:?}", Address::from(event.1)),
                        &format!("{:?}", Address::from(event.2)),
                        &format!("{:?}", Address::from(event.3)),
                        false,
                        false,
                        timestamp,
                        true,
                    )
                    .await
            }
            GroundCycleNoCryptoContractEvents::TakeoffFilter(event) => {
                database
                    .save_landing(
                        event.0.as_u32(),
                        &format!("{:?}", Address::from(event.1)),
                        &format!("{:?}", Address::from(event.2)),
                        &format!("{:?}", Address::from(event.3)),
                        true,
                        false,
                        timestamp,
                        true,
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

impl Default for DatabaseStats {
    fn default() -> Self {
        Self {
            landings: 0,
            amount: "0".to_string(),
        }
    }
}

#[derive(Clone)]
struct Database {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    async fn new(dsn: String) -> Result<Self, Error> {
        // Create file if not exists to be able to open and migrate.
        let file_name = dsn.split(':').collect::<Vec<&str>>()[1];
        if let Err(e) = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .create_new(true)
            .open(file_name)
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
        no_crypto: bool,
    ) -> Result<(), Error> {
        // If it is not takeoff or rejected it is approved landing so we can update stats.
        if is_taken_off || is_rejected {
            let drone_stats = self.get_stats(drone).await?;
            let station_stats = self.get_stats(station).await?;
            let landlord_stats = self.get_stats(landlord).await?;
            let (
                (drone_landings, drone_amount),
                (station_landings, station_amount),
                (landlord_landings, landlord_amount),
            ) = if no_crypto {
                (
                    (drone_stats.landings + 1, drone_stats.amount),
                    (station_stats.landings + 1, station_stats.amount),
                    (landlord_stats.landings + 1, landlord_stats.amount),
                )
            } else {
                let station_drone_agreement = self.get_agreement(station, drone).await?;
                let station_landlord_agreement = self.get_agreement(station, landlord).await?;
                (
                    (
                        drone_stats.landings + 1,
                        (Decimal::from_str(&drone_stats.amount).map_err(map_decimal_err)?
                            - Decimal::from_str(&station_drone_agreement.amount)
                                .map_err(map_decimal_err)?)
                        .to_string(),
                    ),
                    (
                        station_stats.landings + 1,
                        (Decimal::from_str(&station_stats.amount).map_err(map_decimal_err)?
                            + Decimal::from_str(&station_drone_agreement.amount)
                                .map_err(map_decimal_err)?
                            - Decimal::from_str(&station_landlord_agreement.amount)
                                .map_err(map_decimal_err)?)
                        .to_string(),
                    ),
                    (
                        landlord_stats.landings + 1,
                        (Decimal::from_str(&landlord_stats.amount).map_err(map_decimal_err)?
                            + Decimal::from_str(&station_landlord_agreement.amount)
                                .map_err(map_decimal_err)?)
                        .to_string(),
                    ),
                )
            };
            self.save_stats(drone, drone_landings, &drone_amount).await?;
            self.save_stats(station, station_landings, &station_amount).await?;
            self.save_stats(landlord, landlord_landings, &landlord_amount).await?;
        }

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
        query.push(" order by id desc");
        query.push(" limit ").push_bind(limit).push(" offset ").push_bind(offset);
        Ok(query)
    }

    async fn save_stats(&self, address: &str, landings: u32, amount: &str) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query("insert into stats values (?1, ?2, ?3) on conflict do update set landings = ?2, amount = ?3")
            .bind(address.to_lowercase())
            .bind(landings)
            .bind(amount)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }

    async fn get_stats(&self, address: &str) -> Result<DatabaseStats, Error> {
        let mut conn = self.conn.lock().await;
        match sqlx::query_as::<_, DatabaseStats>("select * from stats where address = ?1")
            .bind(address.to_lowercase())
            .fetch_one(&mut *conn)
            .await
        {
            Ok(stats) => Ok(stats),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(DatabaseStats::default()),
                _ => Err(e.into()),
            },
        }
    }
}

fn map_decimal_err(e: rust_decimal::Error) -> String {
    e.to_string()
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

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        value.0.into()
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

async fn run_api(host: String, port: u16, database: Database) -> Result<(), Error> {
    let app = Router::new()
        .route("/stations", get(get_stations))
        .route("/agreements", get(get_agreements))
        .route("/landings", get(get_landings))
        .route("/stats", get(get_stats))
        .layer(Extension(database))
        .fallback(fallback);
    let addr = format!("{}:{}", host, port);
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
    20
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
        if let Ok(agreement) = database.get_agreement(&internal.station, &internal.drone).await {
            external[0].agreements = Some(vec![agreement.into()])
        }
        if let Ok(agreement) = database.get_agreement(&internal.station, &internal.landlord).await {
            if let Some(mut agreements) = external[0].agreements.take() {
                agreements.push(agreement.into())
            } else {
                external[0].agreements = Some(vec![agreement.into()])
            }
        }
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
