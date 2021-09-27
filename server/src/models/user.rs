use crate::schema::*;
use crate::Connection;
use crate::DbId;
use chrono::NaiveDateTime;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "user"]
pub struct DbUser {
    pub id: DbId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub api_key: Option<String>,
    pub github_access_token: Option<String>,
    pub active: bool,
    pub confirmed_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: DbId,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub confirmed_at: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all(conn: &Connection, limit: i64, offset: i64) -> QueryResult<Vec<User>> {
        user::table
            .limit(limit)
            .offset(offset)
            .select((user::id, user::username, user::email, user::active, user::confirmed_at))
            .load::<User>(conn)
    }

    pub fn delete(conn: &Connection, id: i32) -> QueryResult<usize> {
        let result = diesel::delete(user::table.filter(user::id.eq(id))).execute(conn)?;
        Ok(result)
    }

    // ToDo Roles, Permissions
}
