extern crate jwt;
extern crate hmac;
extern crate sha2;

mod _database;
mod auth_utils;
mod jwt_mod;
mod database_connection;

use _database::{Status, connect_to_db};

use auth_utils::models::Credential;
use auth_utils::login;
use database_connection::Database;


pub fn authenticate(cred : Credential) {

    let conn : Database = Database {
        username: "".to_string(),
        password: "".to_string(),
        host: "".to_string(),
        name: "".to_string()
    } ;
    let status : Status = connect_to_db(conn);
    login(cred);

}