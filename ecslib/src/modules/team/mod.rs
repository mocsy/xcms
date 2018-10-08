pub mod add;
pub mod dashboard;
pub mod data;
pub mod edit;
pub mod list;
pub mod select;

use actix_web::{Form, HttpRequest, HttpResponse};
// use actix_web::middleware::identity::RequestIdentity;
use actix_web::middleware::session::RequestSession;

use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

// use crate::modules::user::UserMeta;
use crate::db::{AppState, DbExecutorError, SQuery};

use crate::modules::access::allowed;
use crate::modules::team::data::Team;
use crate::schema::teams::dsl::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrgId {
    org: i64,
}
pub fn set((req, form): (HttpRequest<AppState>, Form<OrgId>)) -> HttpResponse {
    if let Ok(org) = load(&req, form.org) {
        let perm = allowed(&req, org.access_control_id);
        if perm.read {
            let res = req.session().set("org", org.id);
            if res.is_err() {
                return HttpResponse::ExpectationFailed().finish();
            }
        }
    }
    HttpResponse::Ok().finish()
}

pub fn load(req: &HttpRequest<AppState>, org_id: i64) -> Result<Team, DbExecutorError> {
    let query = teams.filter(id.eq(org_id));
    let select = SQuery {
        select: query,
        phantom: PhantomData::<Team>,
    };
    let orgs = req.state().rdb.send(select).wait()??;
    if let Some(org) = orgs.first() {
        return Ok((*org).clone());
    }
    Err(DbExecutorError::Unknown)
}
