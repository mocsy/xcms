use crate::utils::type_name;
use crate::ColumnInfo;
use heck::{SnakeCase, TitleCase};
use pinnaculum::modules::navigation::InputType;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

pub fn write_edit_rs(table_name: String, db_cols: &[ColumnInfo]) {
    if let Some(id_col) = db_cols.get(0) {
        let id_name = id_col.column_name.clone();
        let result_type_name = type_name(table_name.clone());
        let imports = gen_use(table_name.clone());
        let create_fields =
            gen_create_fields(table_name.clone(), id_name.clone(), db_cols.to_owned());
        let index = gen_index(table_name.clone(), id_name.clone(), result_type_name);
        let index_render = gen_index_render(table_name.clone());
        let save = gen_save(table_name.clone(), id_name.clone(), db_cols.to_owned());

        let pth = Path::new(table_name.as_str());
        create_dir_all(&pth).expect("Couldn't create dir");
        let mut file = File::create(pth.join("edit.rs")).unwrap();
        writeln!(file, "{}", imports).expect("Couldn't write file");
        writeln!(file, "{}", create_fields).expect("Couldn't write file");
        writeln!(file, "{}", index).expect("Couldn't write file");
        writeln!(file, "{}", index_render).expect("Couldn't write file");
        writeln!(file, "{}", save).expect("Couldn't write file");
    }
}

fn gen_use(table_name: String) -> String {
    let type_name = type_name(table_name.clone());
    format!(
        "use actix_web::{{ HttpResponse, HttpRequest, Error, Path, FromRequest, Form }};

use std::marker::PhantomData;
use futures::future::Future;
use diesel::prelude::*;

use pinnaculum::render::Failure;
use pinnaculum::utils::http_ok;
use pinnaculum::db::{{ AppState, SQuery, WQuery }};
use pinnaculum::modules::navigation::{{ ListContext, EditableField, InputType }};

use crate::modules::meta::default_meta;
use crate::schema::{}::dsl::*;
use super::data::{{ {}, {}Data }};\n",
        table_name, type_name, type_name
    )
}

fn gen_create_fields(table_name: String, id_name: String, db_cols: &[ColumnInfo]) -> String {
    let type_name = type_name(table_name.clone());
    let mut res_fn = format!(
        "fn create_fields(data: &[{}]) -> Vec<EditableField> {{
    let mut fields = Vec::new();
    let links = Vec::new();
    for typ in data {{\n",
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

        let input_tp = simple_model(col_name.clone());
        let inp_typ_name = format!("{:?}", input_tp);
        // TODO: implement sentence case
        let title = col_name.to_title_case();
        let name = col_name.to_snake_case();

        let field = format!(
            "        let {}_field = EditableField{{
            input_type: InputType::{},
            title: \"{}\".to_string(),
            name: \"{}\".to_string(),
            value: typ.{}.to_string(),
            links: links.clone(),
            required: false,
        }};
        fields.push({}_field);\n",
            col_name, inp_typ_name, title, name, col_name, col_name,
        );
        res_fn.push_str(field.as_ref());
    }
    res_fn.push_str("    }\n    fields\n}");
    res_fn
}

fn gen_index(table_name: String, id_name: String, result_type: String) -> String {
    format!(
        "pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {{
    let pid = Path::<String>::extract(req).unwrap().parse::<i64>().unwrap();
    let query = {}.filter({}.eq(pid));
    let select = SQuery{{ select: query, phantom: PhantomData::<{}> }};
    if let Ok(thing) = req.state().rdb.send(select)
    .map_err(actix_web::Error::from)
    .wait() {{
        if let Ok(data) = thing {{
            let fields = create_fields(&data);
            return http_ok(index_render(fields));
        }}
    }}
    Ok(HttpResponse::Ok().finish())\n}}",
        table_name, id_name, result_type
    )
}

fn gen_index_render(table_name: String) -> String {
    let type_name = type_name(table_name.clone());
    format!("fn index_render(fields: Vec<EditableField>) -> Result<String, Failure> {{
    let links = crate::menu::default_menu();
    let ctx = ListContext{{ title: \"{}\".to_string(), head: \"{} Editor\".to_string(), search: false }};
    let perm = pinnaculum::modules::navigation::PermissionSet{{
        browse: true,
        read: true,
        edit: true,
        add: true,
        delete: true
    }};
    let list = ructe_block_res!(pinnaculum::templates::navigation::edit, &fields, &ctx, &perm)?;
    let meta = default_meta(\"{} Editor\");
    ructe_page_res!(pinnaculum::templates::navigation::frame, meta, &links, &list)\n}}\n",
        type_name,
        type_name,
        type_name
    )
}

fn gen_save(table_name: String, id_name: String, db_cols: &[ColumnInfo]) -> String {
    let type_name = type_name(table_name.clone());
    let mut res_fn = format!(
        "pub fn save((req, form): (HttpRequest<AppState>, Form<{}Data>),) -> HttpResponse {{
    let pid = Path::<String>::extract(&req).unwrap().parse::<i64>().unwrap();
    //if id == form.{} {{
        let target = {}.filter({}.eq(pid));
        let query = diesel::update(target).set((\n",
        type_name, id_name, table_name, id_name,
    );
    for db_col in db_cols {
        let col_name = db_col.column_name.clone();
        // id can't be changed
        if col_name.eq(&id_name) {
            continue;
        }
        // _id is assumed to be unchangeable as well
        if col_name.ends_with("_id") {
            continue;
        }
        // uuid can't be changed
        if col_name.eq("uuid") {
            continue;
        }

        let upd = format!("        {}.eq(form.{}.clone()),\n", col_name, col_name);
        res_fn.push_str(upd.as_ref());
    }
    let query = format!(
        "        ));
        let upd = WQuery {{
            query,
            phantom: PhantomData::<{}>,
        }};
        let res = req.state().wdb.send(upd)
            .map_err(actix_web::Error::from)
            .wait()
            .ok()
            .unwrap()
            .unwrap();
        debug!(\"{{:?}}\", res);
    //}}
    HttpResponse::Ok().finish()\n}}",
        type_name
    );
    res_fn.push_str(query.as_ref());
    res_fn
}

// TODO replace this with the AI model
fn simple_model(name: String) -> InputType {
    // preprocess
    let name = name.to_lowercase();
    if name.eq("uuid") {
        return InputType::Hidden;
    }
    if name.contains("title") {
        return InputType::Input;
    }
    if name.contains("name") {
        return InputType::Input;
    }
    if name.eq("id") {
        return InputType::Input;
    }

    if name.contains("body") {
        return InputType::TextArea;
    }
    if name.contains("text") {
        return InputType::TextArea;
    }
    if name.contains("preview") {
        return InputType::TextArea;
    }
    if name.contains("content") {
        return InputType::TextArea;
    }

    if name.contains("number") {
        return InputType::Select;
    }
    if name.contains("quantity") {
        return InputType::Select;
    }
    if name.contains("type") {
        return InputType::Select;
    }

    InputType::Input
}
