use crate::database::PoolType;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::handlers::user::InputUser;
use crate::schema::users;
use crate::schema::users::dsl::*;
use actix_web::web::{Json};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

pub fn find(pool: &PoolType, user_id: i32) -> Result<User, diesel::result::Error> {

    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

pub fn get_all_users(pool: &PoolType) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn add_single_user(pool: &PoolType, item: Json<InputUser>) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

pub fn delete_single_user(pool: &PoolType, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use crate::tests::helpers::tests::get_pool;

//     pub fn get_all() -> Result<Vec<User>, diesel::result::Error> {
//         let pool = get_pool();
//         get_all_users(&pool)
//     }

//     #[test]
//     fn it_gets_all_users() {
//         let all = get_all();
//         assert!(all.is_ok());
//     }
// }