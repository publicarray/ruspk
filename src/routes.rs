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

#[get("/package/<id>")]
pub fn get_package_versions(conn: DbConn, id: usize) -> Result<Json<Vec<Version>>, String> {
    use crate::schema::version::dsl::*;

    version.load(&*conn.0).map_err(|err| -> String {
        println!("Error querying package: {:?}", err);
        "Error querying package from the database".into()
    }).map(Json)
}
