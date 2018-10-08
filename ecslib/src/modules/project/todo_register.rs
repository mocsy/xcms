use actix_web::middleware::session::RequestSession;
use actix_web::{Error, Form, FromRequest, HttpRequest, HttpResponse, Path};

use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, Conn, DQuery, WQuery};
use crate::modules::navigation::Link;
use crate::render::Failure;
use crate::utils::http_ok;

use crate::modules::meta::default_meta;
use crate::modules::project::data::Project;
use crate::modules::project::todo::Todo;
use crate::schema::todos::dsl::*;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct Register {
    pub title: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub completed: Option<String>,
}
pub fn save((req, form): (HttpRequest<AppState>, Form<Register>)) -> HttpResponse {
    debug!(
        "todo from: {:?} data:{:?}",
        req.connection_info().remote(),
        form.clone()
    );
    // let params = Path::<(String, String)>::extract(&req).unwrap();
    // let ecs = &params.1;
    if req.match_info().get("id").is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let ecs = req.match_info().get("id").unwrap();
    debug!("ecs str: {}", ecs);
    let ecs = ecs.parse::<i64>().unwrap();
    debug!("ecs id: {}", ecs);
    if let Some(orgid) = req.session().get::<i64>("org").unwrap() {
        debug!("{},{}", orgid, ecs);
        if let Ok(project) = Project::load(&req, orgid, ecs) {
            let project_uuid = project.uuid;
            let email_ = if let Some(email_) = &form.email {
                Some(email_.trim().to_owned())
            } else {
                None
            };
            let phone_ = if let Some(phone_) = &form.phone {
                Some(phone_.trim().to_owned())
            } else {
                None
            };
            let desc = form
                .description
                .clone()
                .unwrap_or(String::default())
                .trim()
                .to_owned();
            let desc = if desc.is_empty() { None } else { Some(desc) };
            let form = Register {
                title: form.title.trim().to_owned(),
                email: email_,
                phone: phone_,
                description: desc,
                completed: form.completed.clone(),
            };
            let present = match form.completed {
                Some(thing) => thing.eq("on"),
                None => false,
            };
            let query = diesel::insert_into(todos).values((
                title.eq(form.title),
                email.eq(form.email),
                phone.eq(form.phone),
                description.eq(form.description),
                project_id.eq(project_uuid),
                completed.eq(present),
            ));
            let ins = WQuery {
                query,
                phantom: PhantomData::<Todo>,
            };
            let res = req
                .state()
                .wdb
                .send(ins)
                .map_err(actix_web::Error::from)
                .wait()
                .ok()
                .unwrap()
                .unwrap();
            debug!("{:?}", res);
            // Ok(HttpResponse::Found().header("location", "../invalid").finish())
        }
    }
    HttpResponse::Found().header("location", "./todo").finish()
    // HttpResponse::Found().body("aaa")
    // http_ok(Ok("ssss".to_string()))
}

fn index_render(project: &Project, reg_data: &Register) -> Result<String, Failure> {
    // let toplinks = crate::menu::default_top_menu();
    let toplinks = Vec::new();
    let links = vec![
        Link::new(
            "Add todo",
            &format!("/project/{}/register", project.projectid),
        ),
        Link::new("Todo list", &format!("/project/{}/todo", project.projectid)),
    ];
    let list = ructe_block_res!(crate::templates::project::todo_register, project, reg_data)?;
    let meta = default_meta("project registration");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let ecs = Path::<String>::extract(&req)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    if let Some(orgid) = req.session().get::<i64>("org")? {
        debug!("{},{}", orgid, ecs);
        if let Ok(project) = Project::load(&req, orgid, ecs) {
            return http_ok(index_render(&project, &Register::default()));
        }
    }
    let org_select = "/org/select".to_owned();
    Ok(HttpResponse::Found()
        .header("location", org_select)
        .finish())
}

pub fn edit_page(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    if req.match_info().get("id").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let ecs = req.match_info().get("id").unwrap().parse::<i64>().unwrap();
    if req.match_info().get("aid").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let aid = req.match_info().get("aid").unwrap().parse::<i64>().unwrap();

    if let Some(orgid) = req.session().get::<i64>("org")? {
        debug!("{},{}", orgid, ecs);
        if let Ok(project) = Project::load(&req, orgid, ecs) {
            if let Ok(att) = Todo::load(&req, aid) {
                let reg = Register {
                    title: att.title,
                    email: att.email,
                    phone: att.phone,
                    description: att.description,
                    completed: None,
                };
                return http_ok(index_render(&project, &reg));
            }
        }
    }
    let org_select = "/org/select".to_owned();
    Ok(HttpResponse::Found()
        .header("location", org_select)
        .finish())
}

pub fn save_todo(
    (req, form): (HttpRequest<AppState>, Form<Register>),
) -> Result<HttpResponse, Error> {
    if req.match_info().get("id").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let ecs = req.match_info().get("id").unwrap().parse::<i64>().unwrap();
    if req.match_info().get("aid").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let aid = req.match_info().get("aid").unwrap().parse::<i64>().unwrap();
    if let Some(orgid) = req.session().get::<i64>("org")? {
        if let Ok(_project) = Project::load(&req, orgid, ecs) {
            // if project uuid would be in a hidden, we could double check here
            let email_ = if let Some(email_) = &form.email {
                Some(email_.trim().to_owned())
            } else {
                None
            };
            let phone_ = if let Some(phone_) = &form.phone {
                Some(phone_.trim().to_owned())
            } else {
                None
            };
            let desc = form
                .description
                .clone()
                .unwrap_or(String::default())
                .trim()
                .to_owned();
            let desc = if desc.is_empty() { None } else { Some(desc) };
            let form = Register {
                title: form.title.trim().to_owned(),
                email: email_,
                phone: phone_,
                description: desc,
                completed: form.completed.clone(),
            };

            let target = todos.filter(id.eq(aid));
            let query = diesel::update(target).set((
                title.eq(form.title),
                email.eq(form.email),
                phone.eq(form.phone),
                description.eq(form.description),
            ));
            let upd = WQuery {
                query,
                phantom: PhantomData::<Todo>,
            };
            let res = req.state().wdb.send(upd).wait().ok().unwrap();
            debug!("{:?}", res);
            let route = "../todo";
            return Ok(HttpResponse::Found().header("location", route).finish());
        }
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn delete_todo_conn(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    if req.match_info().get("id").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let ecs = req.match_info().get("id").unwrap().parse::<i64>().unwrap();
    if req.match_info().get("aid").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let aid = req.match_info().get("aid").unwrap().parse::<i64>().unwrap();
    if let Some(orgid) = req.session().get::<i64>("org")? {
        if let Ok(project) = Project::load(&req, orgid, ecs) {
            let conn = req.state().wdb.send(Conn {}).wait().ok().unwrap().unwrap();
            let res = diesel::delete(todos.filter(id.eq(aid))).execute(&conn);
            debug!("{:?}", res);
            let route = format!("/project/{}/todo", project.projectid);
            return Ok(HttpResponse::Found().header("location", route).finish());
        }
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn delete_todo(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    if req.match_info().get("id").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let ecs = req.match_info().get("id").unwrap().parse::<i64>().unwrap();
    if req.match_info().get("aid").is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let aid = req.match_info().get("aid").unwrap().parse::<i64>().unwrap();
    if let Some(orgid) = req.session().get::<i64>("org")? {
        if let Ok(project) = Project::load(&req, orgid, ecs) {
            let query = diesel::delete(todos.filter(id.eq(aid)));
            let del = DQuery { query };
            let res = req.state().wdb.send(del).wait().ok().unwrap();
            debug!("{:?}", res);
            let route = format!("/project/{}/todo", project.projectid);
            return Ok(HttpResponse::Found().header("location", route).finish());
        }
    }
    Ok(HttpResponse::Ok().finish())
}
