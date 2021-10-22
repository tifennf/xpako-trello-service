use std::{collections::HashMap, convert::Infallible, env, net::SocketAddr};

use axum::{handler::head, response::IntoResponse, Json, Router};
use dotenv::dotenv;
use futures::TryStreamExt;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use serde_json::Value;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let key = env::var("KEY").unwrap();
    let token = env::var("TOKEN").unwrap();
    let board_id = env::var("BOARD_ID").unwrap();
    let url = format!(
        "https://api.trello.com/1/boards/{}?key={}&token={}",
        board_id, key, token
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new().route("/", head(webhook_check).post(manage_webhook));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn webhook_check() -> StatusCode {
    StatusCode::OK
}

async fn manage_webhook(Json(payload): Json<HashMap<String, Value>>) -> StatusCode {
    println!("ok ok ok \n{:#?}", payload);

    StatusCode::OK
}
