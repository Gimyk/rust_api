use serde::{Deserialize, Serialize};

use mongodb::error::Result as MongoResult;
use bson::{Bson, Document};


#[derive(Serialize, Deserialize)]
pub struct User{
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Atlass{
    pub country: String,
    pub city: String,
    pub continent: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStuff{
    pub email: String,
    pub password: String
}

pub fn get_data_string(result: MongoResult<Document>) -> Result<Bson, String> {
    match result {
        Ok(doc) => Ok(Bson::Document(doc)),
        Err(e) => Err(format!("{}", e))
    }
}