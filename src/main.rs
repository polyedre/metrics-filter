use std::env;

use actix_web::{get, HttpResponse, HttpServer, App, web, Result};
use reqwest;
use regex::Regex;

fn filter(input: String, pattern: regex::Regex) -> Result<String, regex::Error> {
    Ok(input
       .lines()
       .filter(|line| !pattern.is_match(line))
       .collect::<Vec<_>>()
       .join("\n"))
}

#[get("/metrics")]
async fn metrics(filtered_pattern: web::Data<regex::Regex>, target_url: web::Data<String>) -> Result<HttpResponse> {
    let res = reqwest::get(target_url.get_ref()).await.unwrap().text().await.unwrap();
    Ok(HttpResponse::Ok().body(filter(res, filtered_pattern.get_ref().clone()).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // Parse ENV variables
    let regex_pattern = env::var("EXCLUDED_METRICS_REGEX").unwrap_or("$^".to_string());
    let target: String = env::var("TARGET_URL").unwrap_or("http://localhost:8080".to_string());
    let listen_port: u16 = env::var("LISTEN_PORT")
        .unwrap_or("9090".to_string())
        .parse()
        .expect("Value of environment variable LISTEN_PORT cannot be converted to u16.");

    // We compute the regex pattern once instead of computing it a each request.
    // Therefore, we need to pass these as argument of the `metrics` function.
    // The data needs to be encapsulated in a web::Data object.
    let regex_pattern = web::Data::new(Regex::new(&regex_pattern).unwrap());
    let target_url = web::Data::new(target);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&regex_pattern))
            .app_data(web::Data::clone(&target_url))
            .service(metrics)
    })
        .bind(("127.0.0.1", listen_port))?
        .run()
        .await
}
