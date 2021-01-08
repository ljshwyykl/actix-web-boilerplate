use crate::handle::api_error::ApiError;
use crate::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let users = users::table.load::<User>(&conn)?;

        // return Err(ApiError{
        //     code:10,
        //     msg:"tt".to_string()
        // });

        Ok(users)

        // Err(ApiError{
        //     code:10,
        //     msg:"tt".to_string()
        // });
        // Ok(users)
    }

    pub fn create(user: UserEntity) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }
}

impl From<UserEntity> for User {
    fn from(user: UserEntity) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
