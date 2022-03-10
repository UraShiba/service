use crate::schema::user_info;

#[derive(Queryable, Debug, Insertable)]
#[table_name = "user_info"]
pub struct UserInfo {
    pub user_id: i32,
    pub user_name: String,
    pub login_name: String,
    pub pass: String,
}
