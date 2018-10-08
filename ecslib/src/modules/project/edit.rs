use actix_web::middleware::session::RequestSession;
use actix_web::{Error, Form, FromRequest, HttpRequest, HttpResponse, Path};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, SQuery, WQuery};
use crate::render::Failure;
use crate::utils::http_ok;

use crate::schema::projects::dsl::*;
// use crate::schema::projects;
use super::data::{Project, ProjectData};
use crate::modules::meta::default_meta;

fn create_fields(data: &[Project]) -> ProjectData {
    let ecs = data.first().unwrap();

    let sdate = if let Some(sdate) = ecs.start_date {
        Some(sdate.to_rfc3339())
    } else {
        None
    };
    let edate = if let Some(enddate) = ecs.end_date {
        Some(enddate.to_rfc3339())
    } else {
        None
    };
    let ecs = ecs.clone();
    ProjectData {
        uuid: ecs.uuid,
        title: ecs.title,
        content: Some(ecs.content),
        ecs_start_date: sdate,
        ecs_end_date: edate,
    }
}
pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let id = Path::<String>::extract(req)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    if let Some(orgid) = req.session().get::<i64>("org")? {
        let query = projects.filter(projectid.eq(id)).filter(team_id.eq(orgid));
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
                let fields = create_fields(&data);
                return http_ok(index_render(fields));
            }
        }
    } else {
        let org_select = "/org/select".to_owned();
        return Ok(HttpResponse::Found()
            .header("location", org_select)
            .finish());
    }
    Ok(HttpResponse::Ok().finish())
}
fn index_render(ecs: ProjectData) -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = crate::menu::default_menu();

    let list = ructe_block_res!(crate::templates::project::edit, &ecs)?;
    let mut meta = default_meta("Project Editor");
    meta.add_local_css("/static/ecs_web_kit/css/chunk-vendors.99b7ff43.css");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn save((req, form): (HttpRequest<AppState>, Form<ProjectData>)) -> HttpResponse {
    let ecs = Path::<String>::extract(&req)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    if let Some(org) = req.session().get::<i64>("org").unwrap() {
        debug!("Editing {:?}/{:?}", org, ecs);
        let cont = if let Some(cnt) = &form.content {
            cnt.trim().to_owned()
        } else {
            String::new()
        };
        let titl = &form.title.trim().to_owned();

        let sdate = if form.ecs_start_date.is_some() {
            match form
                .ecs_start_date
                .clone()
                .unwrap()
                .parse::<DateTime<Utc>>()
            {
                Ok(dt) => Some(dt),
                Err(e) => {
                    info!("Error parsing start date: {:?}", e);
                    None
                }
            }
        } else {
            None
        };
        let edate = if form.ecs_end_date.is_some() {
            match form.ecs_end_date.clone().unwrap().parse::<DateTime<Utc>>() {
                Ok(dt) => Some(dt),
                Err(e) => {
                    info!("Error parsing date: {:?}", e);
                    None
                }
            }
        } else {
            None
        };
        let target = projects.filter(projectid.eq(ecs)).filter(team_id.eq(org));
        let query = diesel::update(target).set((
            // uuid.eq(form.uuid),
            title.eq(titl.clone()),
            content.eq(cont),
            start_date.eq(sdate),
            end_date.eq(edate),
        ));
        let upd = WQuery {
            query,
            phantom: PhantomData::<Project>,
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
    }
    HttpResponse::Found().header("location", "../list").finish()
}
