use actix_web::middleware::session::RequestSession;
use actix_web::{Error, Form, HttpRequest, HttpResponse, Json};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, SQuery, WQuery};
use crate::render::Failure;
use crate::utils::http_ok;

use super::data::{Project, ProjectData};
use crate::modules::meta::default_meta;
use crate::schema::projects::dsl::*;

pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    http_ok(index_render())
}

fn index_render() -> Result<String, Failure> {
    let ecs = ProjectData::default();
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

pub fn save((req, form): (HttpRequest<AppState>, Form<serde_json::Value>)) -> HttpResponse {
    log::debug!("{:?}", form);
    let form: ProjectData = serde_json::from_value(form.clone()).unwrap();
    if let Some(orgid) = req.session().get::<i64>("org").unwrap() {
        let query = projects
            .filter(team_id.eq(orgid))
            .order(projectid.desc())
            .limit(1);
        let select = SQuery {
            select: query,
            phantom: PhantomData::<Project>,
        };
        let mut new_id = 0i64;
        if let Ok(thing) = req
            .state()
            .rdb
            .send(select)
            .map_err(actix_web::Error::from)
            .wait()
        {
            if let Ok(data) = thing {
                for ecs in data {
                    new_id = ecs.projectid + 1i64;
                }
            }
        }
        debug!("{:?}", new_id);

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
                    info!("Error parsing end date: {:?}", e);
                    None
                }
            }
        } else {
            None
        };
        use diesel::insert_into;
        let query = insert_into(projects).values((
            projectid.eq(new_id),
            team_id.eq(orgid),
            uuid.eq(::uuid::Uuid::new_v4()),
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
    HttpResponse::Found().header("location", "list").finish()
}
