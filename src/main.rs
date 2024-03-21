#![allow(unused)]

use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, IntoResponseParts, Redirect, Response},
};
use axum::extract::{Path, Query};
use axum::Router;
use axum::Form;
use serde::{Deserialize, Serialize};
use axum::Json;
use axum::routing::{get, get_service, post};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    cors::{Any, CorsLayer},
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

    let mware = CorsLayer::new().allow_headers(Any).allow_origin(Any)
        .expose_headers(Any);
    Router::new()
        .route("/", get(handler_index))
        .layer(ServiceBuilder::new().layer(mware))
        .route("/lotto", get(handler_lotto))
        .route("/api/draw", get(handler_draw))
        .route("/signup", get(handler_signup_html).post(handler_signup))
        .route("/login", get(handler_login_html).post(handler_login))
}

// Auth Handlers
async fn handler_login_html() -> impl IntoResponse {
    let tmpl = LoginTmpl{
        dummy_data: &"Dummy".to_string(),
    };
    return Html(tmpl.render().unwrap())
}
async fn handler_login(Form(LoginUser): Form<LoginUser>) -> (impl IntoResponse){
    let user = utils::db::LoginUser {
        username: LoginUser.username,
        password: LoginUser.password,
    };

    let mut headers = HeaderMap::new();
    let error = utils::db::login(&user).await;
    if error.is_err(){
        return(Html("<p style='color: red;'>Login Failed!</p>".to_string()))
    };
    headers.insert("HX-Target", "/".parse().unwrap());
    return (Html("<p style='color: green;'> Login Successful!</p>".to_string()));
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
    
    let redirect = Redirect::to("/");
    (redirect) 
    }

// Index Handler
async fn handler_index() -> impl IntoResponse{
    let tmpl = HomeTmpl{
        dummy_data: &"Dummy".to_string(),
    };
    return Html(tmpl.render().unwrap());    
}

// Lotto Functions
// Todo: Maybe seperate these out into a different file
async fn handler_draw() -> impl IntoResponse{
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
    let tmpl = BallsTmpl{
        balls: &nums,
    };

    return Html(tmpl.render().unwrap());
}

async fn handler_lotto() -> impl IntoResponse {
    
    let template = MyTemplate {
        dummy_data: &"Dummy".to_string(),       
    };
    Html(template.render().unwrap())
}

// Templates
// TODO: Remove dummy data
#[derive(Template)]
#[template(path = "lotto.html", escape = "none")]
pub struct MyTemplate<'a> {
   dummy_data: &'a String, 
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

#[derive(Template)]
#[template(path = "login.html", escape = "none")]
pub struct LoginTmpl<'a>{
    dummy_data: &'a String,
}

#[derive(Template)]
#[template(path = "components/balls.html", escape = "none")]
pub struct BallsTmpl<'a>{
    balls: &'a Vec<i32>,
}

// Types

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    age: i32,
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}
