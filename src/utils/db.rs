// Add line that ignores dead code
#![allow(dead_code, unused_imports)]

use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Row;

pub struct User {
    pub username: String,
    pub age: i32,
    pub password: String,
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
