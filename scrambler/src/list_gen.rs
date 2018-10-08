use crate::typemap::get_rust_type_str;
use crate::utils::type_name;
use crate::ColumnInfo;
use heck::{CamelCase, SnakeCase};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

pub fn write_list_rs(table_name: String, db_cols: &[ColumnInfo]) {
    // let mut data = Vec::new();
    if let Some(id_col) = db_cols.get(0) {
        let id_name = id_col.column_name.clone();
        let result_type_name = type_name(table_name.clone());
        let imports = gen_use(table_name.clone());
        let result_type = gen_result_type(table_name.clone(), id_name.clone(), db_cols);
        let create_list = gen_create_list(table_name.clone(), id_name.clone(), db_cols);
        let index = gen_index(table_name.clone(), id_name, result_type_name);
        let index_render = gen_index_render(table_name.clone());

        let pth = Path::new(table_name.as_str());
        create_dir_all(&pth).expect("Couldn't create dir");
        let mut data_file = File::create(pth.join("data.rs")).unwrap();
        writeln!(data_file, "{}", result_type).expect("Couldn't write file");
        let mut file = File::create(pth.join("list.rs")).unwrap();
        writeln!(file, "{}", imports).expect("Couldn't write file");
        writeln!(file, "{}", create_list).expect("Couldn't write file");
        writeln!(file, "{}", index).expect("Couldn't write file");
        writeln!(file, "{}", index_render).expect("Couldn't write file");
    }
}

fn gen_use(table_name: String) -> String {
    let type_name = type_name(table_name.clone());
    format!(
        "use actix_web::{{ HttpResponse, HttpRequest, Error }};

use std::marker::PhantomData;
use futures::future::Future;
use diesel::prelude::*;

use pinnaculum::render::Failure;
use pinnaculum::utils::http_ok;
use pinnaculum::db::{{ AppState, SQuery }};
use pinnaculum::modules::navigation::{{ Link, ListContext, Permission, Row, Cell, CellContent }};

use crate::modules::meta::default_meta;
use crate::schema::{}::dsl::*;
use super::data::{{ {} }};\n",
        table_name, type_name
    )
}

fn gen_result_type(table_name: String, id_name: String, db_cols: &[ColumnInfo]) -> String {
    let type_name = type_name(table_name.clone());
    let mut res_struct = format!("use crate::schema::{};\n", table_name);
    let deriv = format!(
        "#[derive(Insertable, AsChangeset, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]\n#[table_name = \"{}\"]\npub struct {} {{\n",
        table_name, type_name
    );
    res_struct.push_str(deriv.as_ref());
    for db_col in db_cols {
        // println!("{:?}", db_col);
        let field = format!(
            "pub {}: {},\n",
            db_col.column_name.clone(),
            get_rust_type_str(
                db_col.data_type.clone(),
                db_col.udt_name.clone(),
                "YES".eq(&db_col.is_nullable)
            )
        );
        res_struct.push_str(field.as_ref());
    }
    res_struct.push_str("}\n");

    let deriv = format!(
        "#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct {}Data {{\n",
        type_name
    );
    res_struct.push_str(deriv.as_ref());
    for db_col in db_cols {
        // println!("{:?}", db_col);
        let col_name = db_col.column_name.clone();
        // skip id_name
        if col_name.eq(&id_name) {
            continue;
        }
        // skip _id
        if col_name.ends_with("_id") {
            continue;
        }
        // skip uuid
        if col_name.eq("uuid") {
            continue;
        }

        let field = format!(
            "pub {}: {},\n",
            col_name,
            get_rust_type_str(
                db_col.data_type.clone(),
                db_col.udt_name.clone(),
                "YES".eq(&db_col.is_nullable)
            )
        );
        res_struct.push_str(field.as_ref());
        println!("{}", field);
    }
    res_struct.push_str("}\n");
    res_struct
}

fn gen_create_list(table_name: String, id_name: String, db_cols: &[ColumnInfo]) -> String {
    let type_name = type_name(table_name.clone());
    let mut res_fn = format!(
        "fn create_list(data: &[{}]) -> Vec<Row> {{
    let mut res = Vec::new();
    for ent in data {{\n    let mut cells = Vec::new();\n",
        type_name
    );
    for db_col in db_cols {
        let col_name = db_col.column_name.clone();
        // skip id_name
        if col_name.eq(&id_name) {
            continue;
        }
        // skip _id
        if col_name.ends_with("_id") {
            continue;
        }
        // skip uuid
        if col_name.eq("uuid") {
            continue;
        }

        let cont = format!(
            "    let {}_cont = CellContent::new(ent.{}.to_string());\n",
            col_name, col_name
        );
        res_fn.push_str(cont.as_ref());
        let required = db_col.is_nullable.eq("NO");
        let cell = format!("    let {}_cell = Cell{{ title: \"{}\".to_string(), content: {}_cont, is_nullable: {} }};\n",
            col_name.clone(),
            col_name.to_camel_case(),
            col_name.clone(),
            (!required).to_string()
        );
        res_fn.push_str(cell.as_ref());
        let cells = format!("    cells.push({}_cell);\n", col_name.clone());
        res_fn.push_str(cells.as_ref());
    }
    let row = format!(
        "
    let ed = Link{{
        visual: \"Edit\".to_string(),
        url: format!(\"/{}/{{}}\", ent.{}),
        active: false,
        icon: \"fa-edit\".to_string(),
        clearance: Permission::Edit,
        children: None,
    }};
    let del = Link{{
        visual: \"Delete\".to_string(),
        url: format!(\"/{}/{{}}\", ent.{}),
        active: false,
        icon: \"fa-trash\".to_string(),
        clearance: Permission::Delete,
        children: None,
    }};
    let links = vec![ed, del];
    let row = Row {{cells, links}};
    res.push(row);",
        type_name.to_snake_case(),
        id_name,
        table_name.to_snake_case(),
        id_name
    );
    res_fn.push_str(row.as_ref());
    res_fn.push_str("\n    }\n    res\n}");
    res_fn
}

fn gen_index(table_name: String, id_name: String, result_type: String) -> String {
    format!(
        "pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {{
    let query = {}.filter({}.is_not_null());
    let select = SQuery{{ select: query, phantom: PhantomData::<{}> }};
    if let Ok(thing) = req.state().rdb.send(select)
    .map_err(actix_web::Error::from)
    .wait() {{
        if let Ok(data) = thing {{
            let list = create_list(&data);
            return http_ok(index_render(list));
        }}
    }}
    Ok(HttpResponse::Ok().finish())\n}}",
        table_name, id_name, result_type
    )
}

fn gen_index_render(table_name: String) -> String {
    let type_name = type_name(table_name.clone());
    format!("fn index_render(list: Vec<Row>) -> Result<String, Failure> {{
    let links = crate::menu::default_menu();
    let ctx = ListContext{{ title: \"{}\".to_string(), head: \"List of {}\".to_string(), search: false }};
    let perm = pinnaculum::modules::navigation::PermissionSet{{
        browse: true,
        read: true,
        edit: true,
        add: true,
        delete: true
    }};
    let list = ructe_block_res!(pinnaculum::templates::navigation::table, &list, &ctx, &perm)?;
    let meta = default_meta(\"List of {}\");
    ructe_page_res!(pinnaculum::templates::navigation::frame, meta, &links, &list)\n}}\n",
        type_name,
        table_name,
        type_name
    )
}
