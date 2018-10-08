use actix_web::{Error, HttpRequest, HttpResponse};

use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, SQuery};
use crate::modules::navigation::{Cell, CellContent, Link, ListContext, Permission, Row};
use crate::render::Failure;
use crate::utils::http_ok;

use super::data::Team;
use crate::modules::meta::default_meta;
use crate::schema::teams::dsl::*;

fn create_list(data: &[Team]) -> Vec<Row> {
    let mut res = Vec::new();
    for ent in data {
        let mut cells = Vec::new();

        let title_cont = CellContent::new(ent.title.to_string());
        let title_cell = Cell {
            title: "Title".to_string(),
            content: title_cont,
            is_nullable: false,
        };
        cells.push(title_cell);

        let billing_name_cont = CellContent::new(ent.billing_name.to_string());
        let billing_name_cell = Cell {
            title: "BillingName".to_string(),
            content: billing_name_cont,
            is_nullable: false,
        };
        cells.push(billing_name_cell);

        let ed = Link {
            visual: "Edit".to_string(),
            url: format!("/team/{}", ent.id),
            active: false,
            icon: "fa-edit".to_string(),
            clearance: Permission::Edit,
            children: None,
        };
        let del = Link {
            visual: "Delete".to_string(),
            url: format!("/teams/{}", ent.id),
            active: false,
            icon: "fa-trash".to_string(),
            clearance: Permission::Delete,
            children: None,
        };
        let links = vec![ed, del];
        let row = Row { cells, links };
        res.push(row);
    }
    res
}
pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let query = teams.filter(id.is_not_null());
    let select = SQuery {
        select: query,
        phantom: PhantomData::<Team>,
    };
    if let Ok(thing) = req
        .state()
        .rdb
        .send(select)
        .map_err(actix_web::Error::from)
        .wait()
    {
        if let Ok(data) = thing {
            let list = create_list(&data);
            return http_ok(index_render(list));
        }
    }
    Ok(HttpResponse::Ok().finish())
}
fn index_render(list: Vec<Row>) -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = crate::menu::default_menu();
    let ctx = ListContext {
        title: "Team".to_string(),
        head: "List of teams".to_string(),
        search: false,
    };
    let perm = crate::modules::navigation::PermissionSet {
        browse: true,
        read: true,
        edit: true,
        add: true,
        delete: true,
    };
    let list = ructe_block_res!(crate::templates::navigation::table, &list, &ctx, &perm)?;
    let meta = default_meta("List of Teams");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}
