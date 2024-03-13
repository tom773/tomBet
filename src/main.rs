#![allow(unused)]

use axum::extract::{Path, Query};
use axum::response::Html;
use axum::Router;
use axum::routing::{get, get_service};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use axum::response::IntoResponse;
use serde::Deserialize;
use tower_http::services::ServeDir;
use rand::Rng;
use askama::Template;
use std::slice::Iter;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().merge(routes_hello());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_hello.into_make_service())
		.await
		.unwrap();
}

// Practice Routes
fn routes_hello() -> Router {
    Router::new()
        .route("/lotto", get(handler_lotto))
}

async fn handler_lotto() -> impl IntoResponse {
    let mut nums = vec![];
    let mut counter: i32 = 0;
    while counter < 7 {
        let ball = rand::thread_rng().gen_range(1..44);
        if nums.contains(&ball){
            continue
        }else{
            nums.push(ball);
            counter = counter+1;
        }
    }
    let admin = User {
        name: String::from("Admin"),
        age: 69,
    };
    let template = MyTemplate {
        name: &admin.name,
        age: &admin.age,
        nums: &nums,
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub struct MyTemplate<'a> {
    name: &'a String,
    age: &'a u8,
    nums: &'a Vec<i32>,
}

// Types

struct User {
    name: String,
    age: u8,
}
