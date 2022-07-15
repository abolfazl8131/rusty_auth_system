
use crate::_database::get_user;
use crate::auth_utils::models::Credential;


pub fn login(cred : Credential){
    get_user(cred);
}
pub fn logout(cred : Credential){
    get_user(cred);
}
pub(crate) mod models;