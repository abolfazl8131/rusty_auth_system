use crate::auth_utils::models::Credential;
use crate::jwt_mod;
use crate::_database;
use crate::database_connection;


pub enum Status {
    Connected,
    Interrupted,
}
pub fn connect_to_db(conn : database_connection::Database) -> Status {
    println!("{:#?}",conn.name);
    return Status::Connected;
}


pub fn get_user(cred:Credential){
    let _username = cred.username;
    let _password = cred.password;
    //query database_connection with username so get username and password (hashed)
    //compare two password to return id or not!
    let _is_matched : bool = false;

    // if there is an id then call jwt_generate
    let mut _token;
    match _is_matched {
        true => {_token = jwt_mod::generate_token("".to_string(), "".to_string())},
        false => {}
    }


    }