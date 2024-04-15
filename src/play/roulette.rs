use axum::{Router, response::IntoResponse, http::HeaderMap, response::Html};
use askama::Template;

pub async fn hroul() -> impl IntoResponse {
    let tmpl = RouletteTmpl{
       dummy_data: &"Dummy".to_string(), 
    };
    return Html(tmpl.render().unwrap());
}

#[derive(Template)]
#[template(path = "roulette.html", escape = "none")]
pub struct RouletteTmpl<'a> {
    dummy_data: &'a String,  
}
