use std::net::TcpListener;

use mysql::*;
use mysql::prelude::*;
use pkg::threadpool::ThreadPool;

use crate::app::route::create_routes;
use crate::configs::config;
use crate::usecase::book::book_handler::{new_book_handler, BookHandler, Handler};
use crate::usecase::book::book_service::new_book_service;
use crate::usecase::book::book::Book;
use serde::de::Unexpected::Bool;

mod configs;
mod usecase;
mod app;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    pkg::logger::init().expect("Error init log");
    let cfg = config::ApplicationConfig::load_yaml_config("./config/Settings.yaml".to_string());
    let pool = pkg::database::connect(cfg.uri.clone());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", cfg.port.clone()))
        .expect("Failed to bind address");
    let threads = ThreadPool::new(cfg.thread_capacity);
    println!("HTTP server started at {}", cfg.port.clone());
    for stream in listener.incoming() {
        let stream = stream.expect("Connection failed");
        threads.execute({
            let pool = pool.clone();
            move || {
                let mut conn = pool.get_conn().expect("Error getting conn");
                let book_service = new_book_service(conn);
                let book_handler = new_book_handler(Box::new(book_service));
                let mut boxed_trait: Box<dyn Handler> = Box::new(book_handler);
                create_routes(&stream, &mut *boxed_trait);
            }
        });
    }
}
