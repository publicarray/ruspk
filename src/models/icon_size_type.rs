use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::{Pg, PgValue};
use diesel::deserialize::{self, FromSql};
use diesel::*;
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "icon_size")]
pub struct IconSize;


#[derive(Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression)]
pub enum IconSizeEnum {
    Icon72,
    Icon120,
    Icon256,
}

// https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
impl ToSql<IconSize, Pg> for IconSizeEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            IconSizeEnum::Icon72 => out.write_all(b"72")?,
            IconSizeEnum::Icon120 => out.write_all(b"120")?,
            IconSizeEnum::Icon256 => out.write_all(b"256")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<IconSize, Pg> for IconSizeEnum {
    fn from_sql(bytes: Option<PgValue<'_>>) -> deserialize::Result<Self> {
        match not_none!(bytes).as_bytes() {
            b"72" => Ok(IconSizeEnum::Icon72),
            b"120" => Ok(IconSizeEnum::Icon120),
            b"256" => Ok(IconSizeEnum::Icon256),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
