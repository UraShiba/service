use crate::models::UserInfo;
use crate::schema::user_info::dsl::user_info;
use crate::utils::establish_connection;
use diesel::prelude::*;

pub fn insert(user: UserInfo) {
    let conn = establish_connection();

    diesel::insert_into(user_info)
        .values(&user)
        .execute(&conn)
        .expect("Error saving new Users");
}
