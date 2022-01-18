use std::time::SystemTime;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono::MIN_DATETIME;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Book {
    pub serial_id: String,
    pub title: String,
    pub author: String,
    //pub release: NaiveDate,
}

impl Book {
    pub fn book_to_response(&self) -> BookResponse {
        BookResponse {
            serial_id: self.serial_id.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            //release: Utc.from_utc_date(&self.release).and_hms(0, 0, 0),
        }
    }
}

impl Default for Book {
    fn default() -> Book {
        Book {
            serial_id: Uuid::new_v4().to_string(),
            title: "".to_string(),
            author: "".to_string(),
            //release: Utc::now().naive_utc().date(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookRequest {
    pub serial_id: String,
    pub title: String,
    pub author: String,
    //pub release: DateTime<Utc>,
}

impl BookRequest {
    pub fn request_to_book(&self) -> Book {
        Book {
            serial_id: self.serial_id.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            //release: self.release.naive_utc().date(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookResponse {
    pub serial_id: String,
    pub title: String,
    pub author: String,
    //pub release: DateTime<Utc>,
}
