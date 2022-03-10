use crate::models::UserInfo;
use crate::schema::user_info::dsl::user_info;
use crate::utils::establish_connection;
use diesel::prelude::*;

pub fn query_all() {
    let conn = establish_connection();

    let results = user_info
        .load::<UserInfo>(&conn)
        .expect("Error loading customer");

    // 結果を表示
    for r in results {
        println!("{:?}", r);
    }
}
pub fn query(id: i32) {
    let conn = establish_connection();

    let results = user_info
        .find(id)
        .load::<UserInfo>(&conn)
        .expect("Error loading customer");

    // 結果を表示
    for r in results {
        println!("{:?}", r);
    }
}
