#![allow(proc_macro_derive_resolution_fallback)]

pub mod list;
pub mod login;
pub mod register;
pub mod restrict;
pub mod token;

use crate::db::{AppState, SQuery};
use crate::schema::{user_meta, user_pwd, users};
use ::uuid::Uuid;
use actix_web::HttpRequest;
use chrono::{DateTime, Utc};
use futures::future::Future;

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub uuid: Uuid,
}

#[derive(Insertable, Queryable, Associations, Debug, Serialize, Deserialize, Clone)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "user_meta"]
pub struct UserMeta {
    pub user_id: i64,
    pub display: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub phone: String,
    pub frozen: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "user_pwd"]
pub struct UserPwd {
    pub id: i64,
    pub user_id: i64,
    pub pw_hash: String,
    pub frozen: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum UserLoadError {
    DatabaseError(diesel::result::Error),
    MailBoxError(actix::MailboxError),
    NoSuchUserError,
}
impl From<diesel::result::Error> for UserLoadError {
    fn from(error: diesel::result::Error) -> Self {
        UserLoadError::DatabaseError(error)
    }
}
impl From<actix::MailboxError> for UserLoadError {
    fn from(error: actix::MailboxError) -> Self {
        UserLoadError::MailBoxError(error)
    }
}

impl UserMeta {
    pub fn load(req: &HttpRequest<AppState>, usr_email: String) -> Result<UserMeta, UserLoadError> {
        use crate::schema::user_meta::dsl::*;
        use diesel::prelude::*;
        use std::marker::PhantomData;
        let query = user_meta.filter(email.eq(usr_email));
        let select = SQuery {
            select: query,
            phantom: PhantomData::<UserMeta>,
        };
        let usr_metas = req.state().rdb.send(select).wait()??;
        if let Some(usr) = usr_metas.first() {
            return Ok((*usr).clone());
        }
        Err(UserLoadError::NoSuchUserError)
    }
}
