use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_name: &str) -> Result<String> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn str_to_u64(str: &str) -> u64 {
    // trace!("str_to_u64: {}",  str.parse::<u64>().unwrap());
    str.parse().unwrap()
}

#[derive(Deserialize)]
pub struct Paginate {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

/// retrieve HTTP GET Parameters for pagination
extern crate serde_derive;
extern crate serde_qs as qs;
pub fn paginate_qs(query_str: &str) -> (i64, i64) {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Parameters {
        page: Option<i64>,
        size: Option<i64>,
    }

    // if strings are found use defaults
    let params: Parameters = qs::from_str(query_str).unwrap_or(Parameters{ page: Some(0), size: Some(20) });
    let mut offset = params.page.unwrap_or(1); //defaults if not provided
    let mut limit = params.size.unwrap_or(20); //defaults if not provided

    // check fot negatives
    if offset <= 0 {
        offset = 1
    }
    if limit <= 0 {
        limit = 1
    // check that query is reasonable and does not overload the database
    } else if limit > 60 {
        limit = 60
    }
    return (limit, (offset-1) * limit)
}
