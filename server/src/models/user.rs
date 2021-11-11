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
    ) -> Result<UserWithKey> {
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

                // let roles = UserRole::belonging_to(&user).inner_join(role::table)
                let roles = UserRole::belonging_to(&user)
                    .load::<(i32, i32)>(conn)?;
                    // .grouped_by(&user);
                // let a = user
                //     .into_iter()
                //     .zip(roles)
                //     .map(|(b, ba_a)| {
                //         // move data from BuildTmp to Build struct
                //         DbBuild::b_create_build(
                //             b,
                //             ba_a.into_iter()
                //                 // drop BuildArchitecture and only get Architecture.code
                //                 .map(|(_, a)| a.code)
                //                 .collect::<Vec<_>>(),
                //         )
                //     })
                //     .collect::<Vec<_>>();

                dbg!(&roles );
                if &roles.len() > &0 { // user has a role
                    return Ok(user);
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
                return Ok(user);
            }
        }
        // else error
        return Err(diesel::result::Error::NotFound.into());
    }
    // ToDo Roles, Permissions
}
