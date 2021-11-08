use crate::Connection;
use crate::DbId;
use crate::{schema::*, utils};
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
    pub fn find_all(conn: &Connection, limit: i64, offset: i64, search_term: String) -> QueryResult<Vec<User>> {
        user::table
            .order(user::id.desc())
            .filter(user::username.ilike(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .select((user::id, user::username, user::email, user::active, user::confirmed_at))
            .load::<User>(conn)
    }

    pub fn delete(conn: &Connection, id: DbId) -> QueryResult<usize> {
        let result = diesel::delete(user::table.filter(user::id.eq(id))).execute(conn)?;
        Ok(result)
    }

    pub fn validate_api_key(conn: &Connection, key: String) -> QueryResult<User> {
        user::table.filter(user::api_key.eq(key).and(user::active.eq(true)))
            .select((user::id, user::username, user::email, user::active, user::confirmed_at))
            .first::<User>(conn)
    }

    // ToDo Roles, Permissions
}
