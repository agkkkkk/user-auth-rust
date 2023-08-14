use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::token::Token;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetail {
    pub firstname: String,
    pub lastname: Option<String>,
    pub dateofbirth: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub userdata: UserDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRegister {
    pub result: Option<UserData>,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLogin {
    // pub result: Option<UserData>,
    pub status: String,
    pub message: Token,
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "Password Invalid")]
    InvalidPassword,
    #[display(fmt = "No User found")]
    NoUserFound,
    #[display(fmt = "Failed to generate access token")]
    FailedToGenerateAccessToken,
}
