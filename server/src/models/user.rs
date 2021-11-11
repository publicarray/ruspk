use crate::{Connection, models::{DbRole, UserRole}};
use crate::DbId;
use crate::{schema::*, utils};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use bcrypt::verify;
extern crate bcrypt;
use anyhow::{Result};

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "user"]
pub struct UserWithKey {
    pub id: DbId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub active: bool,
    pub api_key: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
}
impl UserWithKey {
    fn remove_password(&mut self) { // FixMe
        self.password = "".to_string();
    }
}

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "user"]
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
            .filter(user::username.like(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .select((user::id, user::username, user::email, user::active, user::confirmed_at))
            .load::<User>(conn)
    }

    pub fn delete(conn: &Connection, id: DbId) -> QueryResult<usize> {
        let result = diesel::delete(user::table.filter(user::id.eq(id))).execute(conn)?;
        Ok(result)
    }

    // Auth
    pub fn validate_api_key(conn: &Connection, key: String) -> QueryResult<User> {
        user::table
            .filter(user::api_key.eq(key).and(user::active.eq(true)))
            .select((user::id, user::username, user::email, user::active, user::confirmed_at))
            .first::<User>(conn)
    }

    pub fn login(
        conn: &Connection,
        username: &Option<String>,
        email: &Option<String>,
        password: &String,
    ) -> Result<(UserWithKey, Vec<String>)> {
        // let hashed_password = hash(password, 12)?;
        // debug!("{:?}", hashed_password);
        if let Some(email) = email {
            debug!("{:?}", email);

            let mut user = user::table
                .filter(user::email.eq(email).and(user::active.eq(true)))
                .select((user::id, user::username, user::password, user::email, user::active, user::api_key, user::confirmed_at))
                .first::<UserWithKey>(conn)?;
            let valid = verify(password, &user.password)?;
            if valid {
                user.remove_password();
                // let roles = UserRole::belonging_to(&user).load::<UserRole>(conn)?;

                // let roles = UserRole::belonging_to(&user).inner_join(role::table).load::<(UserRole, DbRole)>(conn)?;
                let roles = UserRole::belonging_to(&user).inner_join(role::table)
                    .select((role::id, role::name, role::description)).load::<DbRole>(conn)?;
                // let roles = UserRole::belonging_to(&user).inner_join(user::table).load::<(UserRole, DbUser)>(conn)?;
                dbg!(&roles);

                // [(role_ + role.name).to_uppercase()]
                // let role_str = roles.into_iter().map(|x| "role_".to_owned() + x.name.as_str()).collect::<Vec<_>>();
                let role_str = roles.into_iter().map(|x| ("role_".to_owned() + x.name.as_str()).to_uppercase()).collect::<Vec<_>>();
                dbg!(&role_str);
                // let me = "hi".to_string().concat("me".to_string());

                if role_str.len() > 0 { // user has at least one role
                    return Ok((user, role_str));
                }
            }
        } else if let Some(username) = username {
            debug!("{:?}", username);
            let mut user = user::table
                .filter(user::username.eq(username).and(user::active.eq(true)))
                .select((user::id, user::username, user::password, user::email, user::active, user::api_key, user::confirmed_at))
                .first::<UserWithKey>(conn)?;
            let valid = verify(password, &user.password)?;
            if valid {
                user.remove_password();
                let roles = UserRole::belonging_to(&user).inner_join(role::table)
                    .select((role::id, role::name, role::description)).load::<DbRole>(conn)?;
                let role_str = roles.into_iter().map(|x| ("role_".to_owned() + x.name.as_str()).to_uppercase()).collect::<Vec<_>>();
                if role_str.len() > 0 { // user has at least one role
                    return Ok((user, role_str));
                }
            }
        }
        // else error
        return Err(diesel::result::Error::NotFound.into());
    }
    // ToDo Roles, Permissions
}
