use jwt::{SignWithKey, Error};
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::hmac::Hmac;
use crate::hmac::digest::KeyInit;


pub enum JWT {

}

pub fn generate_token(user_id:String , username:String) -> Result<String, Error> {
    let mut key : Hmac<Sha256> = Hmac::new_from_slice(b"sth").expect("errrrrrr");
    let mut claims = BTreeMap::new();
    claims.insert("id",user_id);
    claims.insert("username", username);
    let token = claims.sign_with_key(&key);
    return token;
}
pub fn refresh(){

}