// Add line that ignores dead code
#![allow(dead_code, unused_imports)]

use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub age: i32,
    pub password: String,
    pub balance: i32,
}

pub struct LoginUser {
    pub username: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LottoNums {
    pub user: String,
    pub nums: Vec<i32>,
}
#[derive(Serialize)]
pub struct FetchNums {
    pub username: String,
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct FetchedNums {
    pub nums: Vec<i32>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Balance {
    pub bal: i32,
}

pub async fn fetchNums(nums: &FetchNums) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = "postgres://tommy:pass@localhost:5432/tommy";
    let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
    let query = "SELECT ticket FROM tickets WHERE username = $1";
    let row = sqlx::query(query)
        .bind(&nums.username)
        .fetch_one(&mut pool)
        .await?;
    let nums = row.get(0);
    Ok(nums)
}

pub async fn insertNums(nums: &LottoNums) -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://tommy:pass@localhost:5432/tommy";
    let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
    let query = "UPDATE tickets SET ticket=$2 WHERE username = $1";
    sqlx::query(query)
        .bind(&nums.user)
        .bind(&nums.nums)
        .execute(&mut pool)
        .await?;
    println!("-- LOTTO NUMS INSERTED --: {:?}", nums.nums);
    Ok(())
}

pub async fn login(user: &LoginUser) -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://tommy:pass@localhost:5432/tommy";
    let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
    let query = "SELECT * FROM usert WHERE username = $1 AND password = $2"; 
    let row = sqlx::query(query)
        .bind(&user.username)
        .bind(&user.password)
        .fetch_one(&mut pool)
        .await?;
    println!("-- USER LOGGED IN --: Name: {}", user.username);
    Ok(())
}

pub async fn create(user: &User)-> Result<(), Box<dyn std::error::Error>> { 
    
    let url = "postgres://tommy:pass@localhost:5432/tommy";
    let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
    let query = "INSERT INTO usert (username, age, password, balance) VALUES ($1, $2, $3, $4)";

    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.age)
        .bind(&user.password)
        .bind(&user.balance)
        .execute(&mut pool)
        .await?;
    println!("-- USER CREATED --: Name: {}, Age: {}\n", user.username, user.age);
    Ok(())
}

pub async fn alterBal(amt: i32) -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://tommy:pass@localhost:5432/tommy";
    let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
    let query = "UPDATE usert SET balance = balance + $1 WHERE username = 'Tommy'";
    sqlx::query(query)
        .bind(amt)
        .execute(&mut pool)
        .await?;
    Ok(())
}

pub async fn getBal() -> Result<i32, Box<dyn std::error::Error>> {
    let url = "postgres://tommy:pass@localhost:5432/tommy";
    let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
    let query = "SELECT balance FROM usert WHERE username = 'Tommy'";
    let row = sqlx::query(query)
        .fetch_one(&mut pool)
        .await?;
    let bal: i32 = row.get(0);
    Ok(bal)
}
