use actix_web::web::{self, Data, Json};
use actix_web::{post, HttpResponse};
use diesel::result::Error;

use bcrypt::{hash, DEFAULT_COST};

// use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
// use diesel::query_dsl::QueryDsl;
use diesel::{ Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::schema::users;
use crate::response::{ResponseLogin, ResponseRegister, UserData, UserDetail};
use crate::{DBConnection, DBPool};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn to_db_user(&self) -> UserDB {
        let hashed_password = bcrypt::hash(self.password.clone(), DEFAULT_COST).unwrap();

        UserDB {
            firstname: self.first_name.clone(),
            lastname: self.last_name.clone(),
            dateofbirth: self.date_of_birth.clone(),
            email: self.email.clone(),
            password: hashed_password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[diesel(table_name = users)]
#[derive(Queryable, Insertable, Debug)]

pub struct UserDB {
    pub firstname: String,
    pub lastname: String,
    pub dateofbirth: String,
    pub email: String,
    pub password: String,
}

impl UserDB {
    pub fn to_user(&self) -> User {
        User {
            first_name: self.firstname.clone(),
            last_name: self.lastname.clone(),
            date_of_birth: self.dateofbirth.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }

    pub fn to_user_details(&self) -> UserDetail {
        UserDetail {
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            dateofbirth: self.dateofbirth.clone(),
            email: self.email.clone(),
        }
    }
}

pub fn create_user(userdb: UserDB, conn: &mut DBConnection) -> Result<ResponseRegister, Error> {
    use crate::schema::users::dsl::*;

    let user = userdb.to_user_details();
    let _ = diesel::insert_into(users).values(&userdb).execute(conn);

    Ok(ResponseRegister {
        result: UserData { userdata: user },
        status: "SUCCESS".to_string(),
        message: "User successfully registered".to_string(),
    })
}



#[post("/user/register")]
async fn register(data: Json<User>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Cannot create connection");

    let user = web::block(move || create_user(data.to_db_user(), &mut conn)).await;
    let res = user.unwrap().unwrap();

    HttpResponse::Ok().json(res)
}

