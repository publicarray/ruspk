use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{Package, Version};
// use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/package")]
pub fn list_packages(conn: DbConn) -> Result<Json<Vec<Package>>, String> {
    use crate::schema::package::dsl::*;

    package.load(&conn.0).map_err(|err| -> String {
        println!("Error querying package: {:?}", err);
        "Error querying package from the database".into()
    }).map(Json)
}

#[get("/package/<num>")]
pub fn get_package_version(conn: DbConn, num: u64) -> Result<Json<Vec<Version>>, String> {
    use crate::schema::version::dsl::*;
    version.filter(package_id.eq(num)).load(&*conn.0).map_err(|err| -> String {
        println!("Error querying package: {:?}", err);
        "Error querying package from the database".into()
    }).map(Json)
}
