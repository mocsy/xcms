use actix_web::middleware::session::RequestSession;
use actix_web::{Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, SQuery};
use crate::modules::navigation::{Cell, CellContent, Link, ListContext, Permission, Row};
use crate::render::Failure;
use crate::utils::http_ok;

use super::data::Project;
use crate::modules::meta::default_meta;
use crate::schema::projects::dsl::*;

fn create_list(data: &[Project], org: i64) -> Vec<Row> {
    let mut res = Vec::new();
    debug!("Listing org:{}, data:{:?}", org, data);
    for ent in data {
        let mut cells = Vec::new();

        let title_cont = CellContent::new(ent.title.to_string());
        let title_cell = Cell {
            title: "Title".to_string(),
            content: title_cont,
            is_nullable: false,
        };
        cells.push(title_cell);

        if let Some(startdate) = ent.start_date {
            let start_date_cont = CellContent::new(startdate.to_string());
            let start_date_cell = Cell {
                title: "Start Date".to_string(),
                content: start_date_cont,
                is_nullable: true,
            };
            cells.push(start_date_cell);
        }
        if let Some(enddate) = ent.end_date {
            let end_date_cont = CellContent::new(enddate.to_string());
            let end_date_cell = Cell {
                title: "End Date".to_string(),
                content: end_date_cont,
                is_nullable: true,
            };
            cells.push(end_date_cell);
        }

        let ed = Link {
            visual: "todo".to_string(),
            url: format!("/project/{}/todo", ent.projectid),
            active: false,
            icon: "fa-chair".to_string(),
            clearance: Permission::Edit,
            children: None,
        };
        let links = vec![ed];
        let row = Row { cells, links };
        res.push(row);
    }
    res
}
pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    // TODO remove when not needed
    let _res = req.session().set("org", 1);
    if let Ok(orgids) = std::env::var("todo_ORG") {
        let orgid = orgids
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("{} must be int", "todo_ORG"));
        let _res = req.session().set("org", orgid);
        let query = projects.filter(team_id.eq(orgid));
        let select = SQuery {
            select: query,
            phantom: PhantomData::<Project>,
        };
        if let Ok(thing) = req
            .state()
            .rdb
            .send(select)
            .map_err(actix_web::Error::from)
            .wait()
        {
            if let Ok(data) = thing {
                let list = create_list(&data, orgid);
                return http_ok(index_render(list));
            }
        }
    } else {
        return Ok(HttpResponse::SeeOther().body("todo service can not determine the team to use!"));
    }
    Ok(HttpResponse::Ok().finish())
}
fn index_render(list: Vec<Row>) -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = Vec::new();
    let ctx = ListContext {
        title: "Project".to_string(),
        head: "List of projects".to_string(),
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
    let meta = default_meta("List of Project");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}
