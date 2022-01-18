use std::collections::HashMap;
use std::convert::Infallible;
use std::io::{Cursor, ErrorKind};
use std::net::TcpStream;
use std::str::FromStr;

use async_trait::async_trait;
use mysql::{Conn, Error, Params, params, Pool, PooledConn, Row, Statement, TxOpts};
use mysql::prelude::Queryable;
use pkg::query::{build_delete, build_insert, build_list};
use pkg::response::json;

use crate::usecase::book::book::Book;
use crate::usecase::book::book_filter::ListOptions;

#[async_trait]
pub trait Service {
    fn sql_list(&mut self, stream: &TcpStream, table: String) -> Result<Vec<Book>, Error>;
    fn delete(&mut self, stream: &TcpStream, table: String, id: String) -> Result<(), Error>;
    fn add(&mut self, stream: &TcpStream, table: String, book: Book) -> Result<(), Error>;
}

pub struct BookService {
    connection: PooledConn,
}

pub fn new_book_service(pc: PooledConn) -> BookService {
    BookService {
        connection: pc,
    }
}

//#[async_trait]
impl Service for BookService {
    fn sql_list(&mut self, stream: &TcpStream, table: String) -> Result<Vec<Book>, Error> {
        let q = build_list(table);
        let result = self.connection
            .query_map(
                q,
                |mut row: Row| {
                    Book {
                        serial_id: row.take("serial_id").unwrap(),
                        title: row.take("title").unwrap(),
                        author: row.take("author").unwrap(),
                        //release:  row.get("release").unwrap(),
                    }
                },
            );
        result
    }

    fn delete(&mut self, stream: &TcpStream, table: String, id: String) -> Result<(), Error> {
        let q = build_delete(table, "serial_id".to_string(), id);
        let mut tx = self.connection.start_transaction(TxOpts::default()).unwrap();
        let exec = tx.exec_drop(q.as_str(), Params::Empty);
        match exec {
            Ok(_) => tx.commit(),
            Err(_) => tx.rollback()
        }
    }

    fn add(&mut self, stream: &TcpStream, table: String, book: Book) -> Result<(), Error> {
        let q = build_insert(table);
        let mut tx = self.connection.start_transaction(TxOpts::default()).unwrap();
        let p = params! {
                    "serial_id" => book.serial_id,
                    "title" => book.title,
                    "author" => book.author,
                };
        let exec = tx.exec_drop(q.as_str(), p);
        match exec {
            Ok(_) => tx.commit(),
            Err(_) => tx.rollback()
        }
    }
}
