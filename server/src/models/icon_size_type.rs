use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;
use std::io::Write;

#[derive(SqlType)]
#[diesel(postgres_type(name = "icon_size"))]
pub struct IconSize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = IconSize)]
pub enum IconSizeEnum {
    Icon72,
    Icon120,
    Icon256,
}

// https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
// https://github.com/diesel-rs/diesel/blob/master/guide_drafts/custom_types.md
impl ToSql<IconSize, Pg> for IconSizeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        // type IsNull = diesel::sql_types::is_nullable::NotNull;
        match *self {
            IconSizeEnum::Icon72 => out.write_all(b"72")?,
            IconSizeEnum::Icon120 => out.write_all(b"120")?,
            IconSizeEnum::Icon256 => out.write_all(b"256")?,
        }
        Ok(IsNull::No)
    }
}

// https://docs.diesel.rs/diesel/deserialize/trait.FromSql.html
impl FromSql<IconSize, Pg> for IconSizeEnum {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();
        match bytes {
            b"72" => Ok(IconSizeEnum::Icon72),
            b"120" => Ok(IconSizeEnum::Icon120),
            b"256" => Ok(IconSizeEnum::Icon256),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
