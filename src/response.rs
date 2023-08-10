use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetail {
    pub firstname: String,
    pub lastname: String,
    pub dateofbirth: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub userdata: UserDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRegister {
    pub result: UserData,
    pub status: String,
    pub message: String,
}

pub struct ResponseLogin {
    pub result: UserData,
    pub status: String,
    pub message: String,
}
