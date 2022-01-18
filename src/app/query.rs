use std::fmt::Display;

pub fn build_list(table: String) -> String {
    format!("SELECT * FROM {}", table)
}

pub fn build_insert(table: String) -> String {
    format!("INSERT INTO {} (serial_id, title, author) VALUES (:serial_id, :title, :author);", table)
}

// pub fn build_delete(table: String, key: String, value: String) -> String {
//     format!("DELETE FROM {} WHERE {} = {};", table, key, value)
// }

pub fn build_delete<K, V>(table: String, key: K, value: V) -> String where K: Display, V: Display {
    format!("DELETE FROM {} WHERE {} = {};", table, key, value)
}
