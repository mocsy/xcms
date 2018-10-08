pub mod control;
pub mod group;
pub mod group_member;
pub mod key;
pub mod rule;

use std::collections::HashMap;
use std::marker::PhantomData;

use actix_web::middleware::identity::RequestIdentity;
use actix_web::middleware::{Middleware, Started};
use actix_web::{FromRequest, HttpRequest, HttpResponse};

use diesel::prelude::*;
use futures::future::Future;

use crate::db::{AppState, DbExecutorError, SQuery, WQuery};
use crate::modules::navigation::PermissionSet;
use crate::modules::user::UserMeta;

use crate::schema::access_control::dsl::*;
use crate::schema::access_group_members::dsl::*;
use crate::schema::access_groups::dsl::*;
use crate::schema::access_rules::dsl::*;
use control::AccessControl;

pub trait AddAllowed {
    fn can_add(req: &HttpRequest<AppState>, usr_id: i64) -> bool;
}

pub fn allowed(req: &HttpRequest<AppState>, entity_id: i64) -> PermissionSet {
    if let Some(pmap) = req.permission() {
        let pmap = pmap.map();
        if let Some(pset) = pmap.get(&entity_id) {
            return (*pset).clone();
        }
    }
    PermissionSet::deny()
}

pub fn access_control_entry(req: &HttpRequest<AppState>) -> Result<AccessControl, DbExecutorError> {
    if let Some(mail) = req.identity() {
        use diesel::insert_into;
        let query =
            insert_into(access_control).values((created_by.eq(mail.clone()), updated_by.eq(mail)));
        let upd = WQuery {
            query,
            phantom: PhantomData::<AccessControl>,
        };
        let res = req.state().wdb.send(upd).wait()??;
        debug!("{:?}", res);
        if let Some(acc) = res.first() {
            return Ok((*acc).clone());
        }
    }
    Err(DbExecutorError::Unknown)
}
pub(crate) fn access_control_entry_with_api_key(
    req: &HttpRequest<AppState>,
    api_key: String,
) -> Result<AccessControl, DbExecutorError> {
    if !api_key.is_empty() {
        use diesel::insert_into;
        let query = insert_into(access_control)
            .values((created_by.eq(api_key.clone()), updated_by.eq(api_key)));
        let upd = WQuery {
            query,
            phantom: PhantomData::<AccessControl>,
        };
        let res = req.state().wdb.send(upd).wait()??;
        debug!("{:?}", res);
        if let Some(acc) = res.first() {
            return Ok((*acc).clone());
        }
    }
    Err(DbExecutorError::Unknown)
}

pub struct PermissionCheck;
impl Middleware<AppState> for PermissionCheck {
    fn start(&self, req: &HttpRequest<AppState>) -> actix_web::Result<Started> {
        if let Some(pmap) = PermissionCheck::from_request(req) {
            req.extensions_mut().insert(PermissionBox(Box::new(pmap)));
            return Ok(Started::Done);
        }
        Ok(Started::Response(HttpResponse::Unauthorized().finish()))
    }
}
impl PermissionCheck {
    fn from_request(req: &HttpRequest<AppState>) -> Option<PermissionMap> {
        if let Some(mail) = req.identity() {
            if let Ok(usr_meta) = UserMeta::load(req, mail) {
                let query = access_rules
                    .inner_join(access_groups.inner_join(access_group_members))
                    .filter(user_id.eq(usr_meta.user_id))
                    .select((
                        crate::schema::access_rules::access_control_id,
                        crate::schema::access_rules::access_type,
                    ));
                let sel = SQuery {
                    select: query,
                    phantom: PhantomData::<(i64, String)>,
                };
                let access = req.state().rdb.send(sel).wait().unwrap().unwrap();
                let accmap: HashMap<_, _> = access.clone().into_iter().collect();

                let mut permmap: HashMap<i64, PermissionSet> = HashMap::new();
                for key in accmap.keys() {
                    let mut types = Vec::new();
                    for perm in &access {
                        if perm.0.eq(key) {
                            types.push(perm.1.clone());
                        }
                    }
                    let mut pm = PermissionSet::deny();
                    for tp in types {
                        match tp.as_ref() {
                            "browse" => pm.browse = true,
                            "read" => pm.browse = true,
                            "edit" => pm.browse = true,
                            "add" => pm.browse = true,
                            "delete" => pm.browse = true,
                            _ => (),
                        }
                    }
                    permmap.insert(*key, pm);
                }
                return Some(PermissionMap(permmap));
            }
        }
        None
    }
}

struct PermissionBox(Box<PermissionMap>);
pub trait RequestPermission {
    /// Get the Permission from the request
    fn permission(&self) -> Option<PermissionMap>;
}
impl<S> RequestPermission for HttpRequest<S> {
    fn permission(&self) -> Option<PermissionMap> {
        if let Some(perm_id) = self.extensions().get::<PermissionBox>() {
            return Some(*(perm_id.0).clone());
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct PermissionMap(HashMap<i64, PermissionSet>);
impl<S> FromRequest<S> for PermissionMap {
    type Config = ();
    type Result = PermissionMap;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        match req.permission() {
            Some(pmap) => pmap,
            None => PermissionMap(HashMap::new()),
        }
    }
}
impl PermissionMap {
    pub fn map(&self) -> &HashMap<i64, PermissionSet> {
        &self.0
    }
}
