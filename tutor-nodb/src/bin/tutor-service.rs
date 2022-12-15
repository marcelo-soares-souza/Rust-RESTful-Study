use std::io;
use std::sync::Mutex;

use actix_web::{App, HttpServer, web};

use routes::*;
use state::AppState;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../models.rs"]
mod models;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    let bind_ip_port = "0.0.0.0:3000";

    println!("[main] Starting Server in {}", bind_ip_port);

    HttpServer::new(app).bind(bind_ip_port)?.run().await
}