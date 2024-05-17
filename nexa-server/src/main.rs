use std::time::SystemTime;

use axum::extract::DefaultBodyLimit;
use axum::http::header::HeaderName;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::{http::StatusCode, Json, Router};
use base64::engine::general_purpose::STANDARD as BASE64_ENGINE;
use base64::Engine;
use bytes::Bytes;
use chrono::SecondsFormat;
use chrono::{DateTime, Utc};
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::header::HeaderValue;
use hyper::header::CONTENT_TYPE;
use hyper::{Method, Request};
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

mod error;
mod photo;
mod video;

use self::error::*;
use self::photo::*;
use self::video::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/v1/session", get(session_get))
        .route("/v1/video", post(video_post))
        .route("/v1/photo", post(photo_post))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024 * 1024));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9890").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn session_get() -> Result<impl IntoResponse, Error> {
    let access_token = get_access_token().await?;
    let authorization =
        HeaderValue::from_str(get_x_forwarded_authorization_header(access_token.as_str()).as_str())
            .unwrap();
    //info!("access token {}", access_token);
    let request = SessionRequest {
        order_id: thread_rng().gen_range(0..9999).to_string(),
        money: Money {
            amount: "1.00".into(),
            currency: "EUR".into(),
        },
        transaction_date_time: current_date_pt(),
        capture_mode: "AUTO".into(),
        redirect_url: "https://web3.staex.io/photo".into(),
        transaction_initiator: "CONSUMER".into(),
        channel: "ecom".into(),
        use_case: "booking".into(),
    };
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(https);
    let body = serde_json::to_string(&request).unwrap();
    //info!("auth {:?}", authorization);
    //info!("body {}", body);
    let https_request: Request<Full<Bytes>> = Request::builder()
        .method(Method::POST)
        .uri("https://egw.int.paymenttools.net/api/v2/sessions")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(X_FORWARDED_AUTHORIZATION, authorization)
        .body(Full::from(body))?;
    let response = client.request(https_request).await?;
    let status = response.status();
    if !status.is_success() {
        let body = response.collect().await?.to_bytes();
        let body = String::from_utf8(body.to_vec());
        return Err(Error::internal(format!(
            "paymenttools session request returned {}: {:?}",
            status, body
        )));
    }
    let body = response.collect().await?.to_bytes();
    //info!("body {:?}", String::from_utf8(body.to_vec()));
    let session: SessionResponse = serde_json::from_slice(body.to_vec().as_slice())?;
    Ok((StatusCode::CREATED, Json(session)))
}

async fn get_access_token() -> Result<String, Error> {
    let request = AuthorizationRequest {
        grant_type: "client_credentials".into(),
        client_id: std::env::var("PAYMENTTOOLS_USERNAME").unwrap(),
        client_secret: std::env::var("PAYMENTTOOLS_PASSWORD").unwrap(),
    };
    let body = serde_urlencoded::to_string(&request).unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(https);
    let https_request: Request<Full<Bytes>> = Request::builder()
        .method(Method::POST)
        .uri("https://auth.int.pci.paymenttools.net/realms/merchants/protocol/openid-connect/token")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .body(Full::from(body))?;
    let response = client.request(https_request).await?;
    let status = response.status();
    if !status.is_success() {
        let body = response.collect().await?.to_bytes();
        let body = String::from_utf8(body.to_vec());
        return Err(Error::internal(format!(
            "paymenttools authorization request returned {}: {:?}",
            status, body
        )));
    }
    let body = response.collect().await?.to_bytes();
    let response: AuthorizationResponse = serde_json::from_slice(body.to_vec().as_slice())?;
    Ok(response.access_token)
}

#[derive(Serialize)]
struct AuthorizationRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct AuthorizationResponse {
    access_token: String,
}

#[derive(Serialize)]
struct SessionRequest {
    #[serde(rename = "orderId")]
    order_id: String,
    money: Money,
    #[serde(rename = "transactionDateTime")]
    transaction_date_time: String,
    #[serde(rename = "captureMode")]
    capture_mode: String,
    #[serde(rename = "redirectUrl")]
    redirect_url: String,
    #[serde(rename = "transactionInitiator")]
    transaction_initiator: String,
    channel: String,
    #[serde(rename = "useCase")]
    use_case: String,
}

#[derive(Serialize)]
struct Money {
    amount: String,
    currency: String,
}

#[derive(Deserialize, Serialize)]
struct SessionResponse {
    id: String,
}

fn current_date_pt() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    now
}

fn _get_authorization_header() -> String {
    let username = std::env::var("PAYMENTTOOLS_USERNAME").unwrap();
    let password = std::env::var("PAYMENTTOOLS_PASSWORD").unwrap();
    let mut authorization = String::new();
    authorization.push_str(username.as_str());
    authorization.push(':');
    authorization.push_str(password.as_str());
    let authorization = BASE64_ENGINE.encode(authorization.as_bytes());
    format!("Basic {}", authorization)
}

fn get_x_forwarded_authorization_header(access_token: &str) -> String {
    format!("Bearer {}", access_token)
}

#[allow(clippy::declare_interior_mutable_const)]
const X_FORWARDED_AUTHORIZATION: HeaderName = HeaderName::from_static("x-forwarded-authorization");
