// use actix_web::http::Error;
use actix_web::web::{self, Data, Json};
use actix_web::{post, HttpResponse};
use diesel::result::Error;
// use std::io::Error;

use bcrypt::{verify, DEFAULT_COST};

use diesel::prelude::*;
// use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl, SelectDsl};
// use diesel::query_dsl::QueryDsl;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::schema::users;
use crate::response::{ResponseLogin, ResponseRegister, UserData, UserDetail};
use crate::{DBConnection, DBPool};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub firstname: String,
    pub lastname: Option<String>,
    pub dateofbirth: Option<String>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn to_db_user(&self) -> UserDB {
        let hashed_password = bcrypt::hash(self.password.clone(), DEFAULT_COST).unwrap();

        UserDB {
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            dateofbirth: self.dateofbirth.clone(),
            email: self.email.clone(),
            password: hashed_password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[diesel(table_name = users)]
#[derive(Queryable, Insertable, Debug)]
pub struct UserDB {
    pub firstname: String,
    pub lastname: Option<String>,
    pub dateofbirth: Option<String>,
    pub email: String,
    pub password: String,
}

impl UserDB {
    pub fn to_user(&self) -> User {
        User {
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            dateofbirth: self.dateofbirth.clone(),
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
        result: Some(UserData { userdata: user }),
        status: "SUCCESS".to_string(),
        message: "User successfully registered".to_string(),
    })
}

pub fn login_user(user: LoginUser, conn: &mut DBConnection) -> Result<ResponseLogin, Error> {
    use crate::schema::users::dsl::*;

    let user_with_password = match users
        .filter(&email.eq(&user.email))
        .select((email, password))
        .first::<LoginUser>(conn)
    {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::NotFound),
    };

    println!("{:?}", user_with_password);

    if !verify(user.password, &user_with_password.unwrap().password).unwrap() {
        return Err(Error::NotFound);
    }

    let user_data = match users
        .filter(&email.eq(&user.email))
        .select((firstname, lastname, dateofbirth, email, password))
        .first::<UserDB>(conn)
    {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::NotFound),
    };

    Ok(ResponseLogin {
        result: Some(UserData {
            userdata: user_data.unwrap().to_user_details(),
            // UserDetail {
            //     firstname: "".to_string(),
            //     lastname: "".to_string(),
            //     dateofbirth: "".to_string(),
            //     email: "".to_string(),
            // },
        }),
        status: "SUCCESS".to_string(),
        message: "Login Successful".to_string(),
    })
}

#[post("/user/register")]
async fn register(data: Json<User>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Cannot create connection");

    if data.password.len() < 8 || data.password.len() > 15 {
        // return Err(CustomError::InvalidPassword);
        return HttpResponse::ExpectationFailed().json("Invalid Password");
    }

    let user = web::block(move || create_user(data.to_db_user(), &mut conn)).await;
    let res = user.unwrap().unwrap();

    HttpResponse::Ok().json(res)
}

#[post("/user/login")]
async fn login(data: Json<LoginUser>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Cannot create connection");

    let temp_data = serde_json::to_string(&data).unwrap();
    let login_data = serde_json::from_str(&temp_data).unwrap();

    let user = web::block(move || login_user(login_data, &mut conn)).await;
    // let res = user.unwrap().unwrap();
    println!("{:?}", user);
    match user {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(err) => HttpResponse::NotFound().json(format!("{}", err)),
    }
}
