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

pub struct FetchNums {
    pub username: String,
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
    let query = "INSERT INTO usert (username, age, password) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.age)
        .bind(&user.password)
        .execute(&mut pool)
        .await?;
    println!("-- USER CREATED --: Name: {}, Age: {}\n", user.username, user.age);
    Ok(())
}
