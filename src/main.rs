#![allow(unused)]

use axum::{
    response::Redirect,
    http::{HeaderMap, StatusCode},
};
use axum::extract::{Path, Query};
use axum::response::Html;
use axum::Router;
use axum::Form;
use serde::{Deserialize, Serialize};
use axum::Json;
use axum::routing::{get, get_service, post};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use axum::response::IntoResponse;
use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use tower::ServiceBuilder;

use rand::Rng;
use askama::Template;
use std::slice::Iter;
use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Row;
mod utils;   
mod middleware;

#[tokio::main]
async fn main() { 
    
    let routes_hello = Router::new().merge(routes_hello())
        .nest_service("/public", ServeDir::new("public"));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_hello.into_make_service())
		.await
		.unwrap();
}

// Practice Routes
fn routes_hello() -> Router {
    Router::new()
        .route("/", get(handler_index))
        .layer(ServiceBuilder::new().layer(crate::middleware::middleware::MWARE))
        .route("/lotto", get(handler_lotto))
        .route("/api/signup", post(handler_signup))
        .route("/signup", get(handler_signup_html))
}

async fn handler_signup_html() -> impl IntoResponse {
    let tmpl = SignupTmpl{
        dummy_data: &"Dummy".to_string(),
    };
    return Html(tmpl.render().unwrap());
}

async fn handler_signup(Form(CreateUser): Form<CreateUser>)-> 
        (impl IntoResponse) 
    {
    let user = utils::db::User {
        username: CreateUser.username,
        age: CreateUser.age,
        password: CreateUser.password,
    };
    utils::db::create(&user).await.unwrap();
    println!("Successfully created user!");
    
    let mut redirect = HeaderMap::new();
    redirect.insert("HX-Redirect", "/".parse().unwrap());

    (redirect) 
    }

async fn handler_index() -> impl IntoResponse{
    let tmpl = HomeTmpl{
        dummy_data: &"Dummy".to_string(),
    };
    return Html(tmpl.render().unwrap());    
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
    let admin = utils::db::User {
        username: String::from("Admin"),
        age: 69,
        password: String::from("1234"),
    };
    let template = MyTemplate {
        name: &admin.username,
        age: &admin.age,
        nums: &nums,
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "lotto.html", escape = "none")]
pub struct MyTemplate<'a> {
    name: &'a String,
    age: &'a i32,
    nums: &'a Vec<i32>,
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub struct HomeTmpl<'a>{
    dummy_data: &'a String,
}

#[derive(Template)]
#[template(path = "signup.html", escape = "none")]
pub struct SignupTmpl<'a>{
    dummy_data: &'a String,
}

// Types

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    age: i32,
    password: String,
}

