#![allow(unused)]
#![allow(non_snake_case)]
use axum::{
    extract,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, IntoResponseParts, Redirect, Response},
};
use utils::db::FetchedNums;
use std::collections::HashSet;
use axum::extract::{Path, Query, Multipart};
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
mod generator;
use std::sync::Mutex;
use tokio::sync::Mutex as TokioMutex;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

#[tokio::main]
async fn main() { 
    
    let routes_hello = Router::new().merge(routes_hello()).merge(routes_lotto())
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
        .route("/signup", get(handler_signup_html).post(handler_signup))
        .route("/login", get(handler_login_html).post(handler_login))
}

fn routes_lotto() -> Router {
    Router::new()
        .route("/lotto", get(handler_lotto))
        .route("/api/draw", get(handler_draw))
        .route("/api/getnums", get(get_nums))
        .route("/api/select-numbers", post(sel_num))
        .route("/api/balupdate", post(bal_update))
        .route("/api/getbal", get(get_bal))
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
        balance: 0,
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
    let mut matches = vec![];
    let selected = get_nums().await;
    for i in selected.nums.iter(){
        if nums.contains(i){
            matches.push(i.to_owned());
        }
    }
    let tmpl = BallsTmpl{
        balls: &nums,
        winners: &matches,
    };
    return Html(tmpl.render().unwrap());
}

async fn get_nums() -> Json<utils::db::FetchedNums>{
    let selected = utils::db::FetchNums{
        username: "Tommy".to_string(), 
    };
    let ticket = utils::db::fetchNums(&selected).await.unwrap()
        .iter()
        .map(|x| x.parse::<i32>()
        .unwrap())
        .collect::<Vec<i32>>();
    let ticket_ = FetchedNums{
        nums: ticket,
    };
    return Json(ticket_); 
}



async fn sel_num(mut multipart: Multipart){ 
    let mut nums = vec![];
    while let Some(mut field) = multipart.next_field().await.unwrap(){
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let data_str = String::from_utf8(data.to_vec()).unwrap(); 
        for i in data_str.split(","){
            nums.push(i.parse::<i32>().unwrap());
        }
        
    }
    let selected = utils::db::LottoNums{
        user: "Tommy".to_string(),
        nums: nums,
    };

    utils::db::insertNums(&selected).await.unwrap();
    
}

async fn handler_lotto() -> impl IntoResponse {
    
    let template = MyTemplate {
        dummy_data: &"Dummy".to_string(),       
    };
    Html(template.render().unwrap())
}

async fn bal_update() {
    
    utils::db::alterBal(30).await.unwrap();
}

async fn get_bal() -> Json<utils::db::Balance>{
    let balance = utils::db::getBal().await.unwrap();
    return Json(utils::db::Balance{
        bal: balance,
    });
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
    winners: &'a Vec<i32>,
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

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Lotto {
    users: String,
    numbers: Vec<i32>,
}
