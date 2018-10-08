use crate::utils::type_name;
use crate::ColumnInfo;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

pub fn write_mod_rs(table_name: String, db_cols: &[ColumnInfo]) {
    if let Some(id_col) = db_cols.get(0) {
        let id_name = id_col.column_name.clone();
        let result_type_name = type_name(table_name.clone());

        let mod_cnt = format!(
            "
pub mod add;
pub mod data;
pub mod edit;
pub mod list;

use self::data::{};
use crate::schema::{}::dsl::*;

use actix_web::HttpRequest;
use futures::future::Future;

use pinnaculum::db::{{AppState, DbExecutorError, SQuery}};

impl {} {{
    pub fn load(req: &HttpRequest<AppState>, cid: i64) -> Result<{}, DbExecutorError> {{
        use diesel::prelude::*;
        use std::marker::PhantomData;
        let query = {}.filter(crate::schema::{}::{}.eq(cid));
        let select = SQuery {{
            select: query,
            phantom: PhantomData::<{}>,
        }};
        let tmp = req.state().rdb.send(select).wait()??;
        if let Some(obj) = tmp.first() {{
            return Ok((*obj).clone());
        }}
        Err(DbExecutorError::Unknown)
    }}
}}",
            result_type_name,
            table_name,
            result_type_name,
            result_type_name,
            table_name,
            table_name,
            id_name,
            result_type_name,
        );

        let pth = Path::new(table_name.as_str());
        create_dir_all(&pth).expect("Couldn't create dir");
        let mut mod_file = File::create(pth.join("mod.rs")).unwrap();
        writeln!(mod_file, "{}", mod_cnt).expect("Couldn't write file");
    }
}
