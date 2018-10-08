//use actix_web::middleware::session::RequestSession;
use actix_web::{Error, Form, FromRequest, HttpRequest, HttpResponse, Path};

use std::marker::PhantomData;
// HttpMessage, Query, Json };
use futures::future::Future;
// use actix_web::AsyncResponder;
use ::uuid::Uuid;
// use pretty_env_logger;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::db::{AppState, DbExecutorError, SQuery, WQuery};
use crate::modules::navigation::Link;
use crate::render::Failure;
use crate::utils::http_ok;

use crate::modules::meta::default_meta;
use crate::modules::project::data::Project;
use crate::schema::todos;
use crate::schema::todos::dsl::*;

fn index_render(data: Vec<Todo>, project: Project) -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    // let toplinks = Vec::new();
    let links = vec![
        Link::new(
            "Add todo",
            &format!("/project/{}/register", project.projectid),
        ),
        Link::new("Todo list", &format!("/project/{}/todo", project.projectid)),
    ];
    let mut list = ructe_block_res!(crate::templates::project::todo, &data, &project)?;
    // let scr = ecs::modules::Script::new("/static/todo.js");
    // list.push_str(scr.as_html()?.as_ref());
    list.push_str(r#"<script src="/static/todo.js" charset="utf-8"></script>"#);
    let mut meta = default_meta("project todo");
    meta.add_local_css("/static/matswitch.css");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let ecs = Path::<String>::extract(req)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    if let Ok(orgids) = std::env::var("todo_ORG") {
        let orgid = orgids
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("{} must be int", "todo_ORG"));
        debug!("{},{}", orgid, ecs);
        if let Ok(project) = Project::load(req, orgid, ecs) {
            let query = todos.filter(project_id.eq(project.uuid));
            // TODO move ordering to clilent side
            let query = query.order(title.asc());

            let select = SQuery {
                select: query,
                phantom: PhantomData::<Todo>,
            };
            let res = req
                .state()
                .rdb
                .send(select)
                .map_err(actix_web::Error::from)
                .wait()
                .ok()
                .unwrap()
                .unwrap();

            debug!("{:?}", project);
            return http_ok(index_render(res, project));
        }
    } else {
        let org_select = "/org/select".to_owned();
        return Ok(HttpResponse::Found()
            .header("location", org_select)
            .finish());
    }
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToggleParams {
    id: i64,
    value: String,
}
pub fn toggle((req, form): (HttpRequest<AppState>, Form<ToggleParams>)) -> HttpResponse {
    debug!(
        "todo from: {:?} data:{:?}",
        req.connection_info().remote(),
        form.clone()
    );
    // let aid = form.id.parse::<i64>().unwrap();
    let value = form.value.parse::<bool>().unwrap();
    let aid = form.id;
    let query = diesel::update(todos.filter(id.eq(aid))).set(completed.eq(value));
    let upd = WQuery {
        query,
        phantom: PhantomData::<Todo>,
    };
    let res = req
        .state()
        .wdb
        .send(upd)
        .map_err(actix_web::Error::from)
        .wait()
        .ok()
        .unwrap()
        .unwrap();
    debug!("{:?}", res);
    // HttpResponse::Found().finish()
    // Ok(HttpResponse::Found().header("location", "../invalid").finish())
    HttpResponse::Found().header("location", "./todo").finish()
    // HttpResponse::Found().body("aaa")
    // http_ok(Ok("ssss".to_string()))
}

// pub fn print_type_of<T>(_: &T) {
//     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
// }
#[derive(
    Insertable,
    AsChangeset,
    Queryable,
    Associations,
    Identifiable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
// #[belongs_to(Project, foreign_key = "project_id")]
#[belongs_to(Project)]
#[table_name = "todos"]
#[primary_key("id")]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub project_id: Uuid,
    pub completed: bool,
    pub completed_at: DateTime<Utc>,
}

impl Todo {
    pub fn load(req: &HttpRequest<AppState>, aid: i64) -> Result<Todo, DbExecutorError> {
        use diesel::prelude::*;
        let query = todos.filter(id.eq(aid));
        let select = SQuery {
            select: query,
            phantom: PhantomData::<Todo>,
        };
        let tmp = req.state().rdb.send(select).wait()??;
        if let Some(obj) = tmp.first() {
            return Ok((*obj).clone());
        }
        Err(DbExecutorError::Unknown)
    }
}
