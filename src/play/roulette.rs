use axum::{Router, response::IntoResponse, http::HeaderMap, response::Html};
use askama::Template;
use rand::Rng;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn hroul() -> impl IntoResponse {
    let tmpl = RouletteTmpl{
       dummy_data: &"Dummy".to_string(), 
    };
    return Html(tmpl.render().unwrap());
}

pub async fn spin() -> Json<ResRoul>{
    let winner = rand::thread_rng().gen_range(0..36);
    let res = ResRoul{
        num: winner,
    };
    return Json(res);
}

#[derive(Template)]
#[template(path = "roulette.html", escape = "none")]
pub struct RouletteTmpl<'a> {
    dummy_data: &'a String,  
}

#[derive(Deserialize, Serialize)]
pub struct ResRoul{
    num: i32,
}
