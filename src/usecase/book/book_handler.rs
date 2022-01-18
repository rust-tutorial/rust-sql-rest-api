use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;

use async_trait::async_trait;
use log::{info, trace, warn};
use mysql::Error;
use crate::app::query::build_list;
use pkg::response::json;
use pkg::ultil::{parse_http_body, ParseBodyError};

use crate::usecase::book::book::{Book, BookRequest};
use crate::usecase::book::book_filter::ListOptions;
use crate::usecase::book::book_service::Service;

//#[async_trait]
pub trait Handler {
    fn sql_list(&mut self, stream: &TcpStream, table: String);
    fn add(&mut self, stream: &TcpStream, body: String);
    fn delete(&mut self, stream: &TcpStream, id: String);
}

pub struct BookHandler {
    service: Box<dyn Service + Send + Sync>,
}

pub fn new_book_handler(s: Box<dyn Service + Send + Sync>) -> BookHandler {
    BookHandler { service: s }
}

//#[async_trait] // Currently async trait is not supported but the restriction will be removed in the future
impl Handler for BookHandler {
    fn sql_list(&mut self, stream: &TcpStream, table: String) {
        let result = self.service.sql_list(stream, table);
        match result {
            Ok(ref v) => json(
                stream,
                serde_json::to_string(&v).unwrap(),
                pkg::status_code::SUCCESS.to_string(),
            ),
            Err(err) => json(
                stream,
                err.to_string(),
                pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
            )
        }
    }
    fn add(&mut self, stream: &TcpStream, body: String) {
        match serde_json::from_str::<BookRequest>(body.as_str()) {
            Ok(book_request) => {
                let book = book_request.request_to_book();
                let add_res = self.service.add(stream, "book".to_string(), book);
                match add_res {
                    Ok(_) => json(
                        stream,
                        "book added".to_string(),
                        pkg::status_code::SUCCESS.to_string(),
                    ),
                    Err(err) => json(
                        stream,
                        err.to_string(),
                        pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
                    )
                }
            }
            Err(err) => json(
                stream,
                err.to_string(),
                pkg::status_code::BAD_REQUEST.to_string(),
            )
        }
    }
    fn delete(&mut self, stream: &TcpStream, id: String) {
        let res = self.service.delete(stream, "book".to_string(), id);
        match res {
            Ok(_) => json(
                stream,
                "deleted successfully".to_string(),
                pkg::status_code::SUCCESS.to_string(),
            ),
            Err(err) => json(
                stream,
                err.to_string(),
                pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
            )
        }
    }
}

