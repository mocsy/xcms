#![allow(proc_macro_derive_resolution_fallback)]
// #[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::*;
use regex::Regex;

use std::env;

mod add_gen;
mod cli;
mod edit_gen;
mod list_gen;
mod mod_gen;
mod typemap;
mod utils;

// "select column_name, data_type, character_maximum_length
// from INFORMATION_SCHEMA.COLUMNS where table_name = '<name of table>';"

#[derive(QueryableByName, Debug, Queryable, PartialEq)]
pub struct ColumnInfo {
    #[sql_type = "Text"]
    column_name: String,
    #[sql_type = "Text"]
    data_type: String,
    #[sql_type = "Text"]
    udt_name: String,
    #[sql_type = "Nullable<BigInt>"]
    character_maximum_length: Option<i64>,
    #[sql_type = "Text"]
    is_nullable: String,
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let table_name = matches
        .value_of("table-name")
        .unwrap_or_else(|| panic!("{} must be set", "table-name"));

    let re = Regex::new(r"(?-u)^[0-9A-Za-z_]{4,32}$").unwrap();
    if re.is_match(table_name) {
        dotenv::dotenv().ok();
        let database_url =
            env::var("DB_READ_URL").unwrap_or_else(|_| panic!("{} must be set", "DB_READ_URL"));
        let conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let res = sql_query(format!("select column_name, data_type, udt_name, character_maximum_length, is_nullable from INFORMATION_SCHEMA.COLUMNS where table_name = '{}'", table_name))
            .load::<ColumnInfo>(&conn)
            .expect("Query failed");
        println!("{:?}", res);

        list_gen::write_list_rs(String::from(table_name), &res);
        edit_gen::write_edit_rs(String::from(table_name), &res);
        add_gen::write_add_rs(String::from(table_name), &res);
        mod_gen::write_mod_rs(String::from(table_name), &res);
    }
}
