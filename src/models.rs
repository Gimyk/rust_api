use serde::{Deserialize, Serialize};

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