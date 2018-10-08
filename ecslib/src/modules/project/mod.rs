pub mod add;
pub mod data;
pub mod edit;
pub mod list;
pub mod todo;
pub mod todo_list;
pub mod todo_register;

// use actix_web::Error;
// use actix_web::FromRequest;
use actix_web::HttpRequest;
// use actix_web::HttpResponse;
// use actix_web::Path;

// use crate::render::Failure;
// use crate::utils::http_ok;

use crate::db::{AppState, DbExecutorError, SQuery};
use crate::modules::project::data::Project;
use crate::schema::projects::dsl::*;
use futures::future::Future;

// use crate::modules::meta::default_meta;

// fn index_render() -> Result<String, Failure> {
//     let meta = default_meta("Project page");
//     ructe_page_res!(ecs::templates::hello, meta, "Project block")
// }

// pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
//     http_ok(index_render())
// }

// fn id_render(id: &str) -> Result<String, Failure> {
//     let meta = default_meta("Project page");
//     ructe_page_res!(ecs::templates::hello, meta, id)
// }

// pub fn id(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
//     let id = Path::<String>::extract(req).unwrap();
//     http_ok(id_render(&id))
// }

impl Project {
    pub fn load(
        req: &HttpRequest<AppState>,
        oid: i64,
        eid: i64,
    ) -> Result<Project, DbExecutorError> {
        use diesel::prelude::*;
        use std::marker::PhantomData;
        let query = projects.filter(projectid.eq(eid)).filter(team_id.eq(oid));
        let select = SQuery {
            select: query,
            phantom: PhantomData::<Project>,
        };
        let tmp = req.state().rdb.send(select).wait()??;
        if let Some(obj) = tmp.first() {
            return Ok((*obj).clone());
        }
        Err(DbExecutorError::Unknown)
    }

    pub fn load_by_uuid(
        req: &HttpRequest<AppState>,
        ecs_uuid: ::uuid::Uuid,
    ) -> Result<Project, DbExecutorError> {
        use diesel::prelude::*;
        use std::marker::PhantomData;
        let query = projects.filter(uuid.eq(ecs_uuid));
        let select = SQuery {
            select: query,
            phantom: PhantomData::<Project>,
        };
        let tmp = req.state().rdb.send(select).wait()??;
        if let Some(obj) = tmp.first() {
            return Ok((*obj).clone());
        }
        Err(DbExecutorError::Unknown)
    }
}
