use crate::db::{AppState, WQuery};
use crate::modules::navigation::Link;
use crate::modules::user::{User, UserMeta, UserPwd};
use crate::render::Failure;
use crate::schema::user_meta::dsl::*;
use crate::schema::user_pwd::dsl::*;
use crate::schema::users::dsl::*;
use crate::utils::http_ok;
use ::uuid::Uuid;
use actix_web::{Error, Form, HttpRequest, HttpResponse};
use argon2rs::{Argon2, Variant};
use diesel::prelude::*;
use futures::future::Future;
use log::{debug, error};
use std::marker::PhantomData;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Register {
    pub email: String,
    pub phone: String,
    pub fname: String,
    pub lname: String,
    pub psw: String,
    pub display: Option<String>,
    pub completed: Option<String>,
    pub invite: Option<String>,
}
pub fn save((req, form): (HttpRequest<AppState>, Form<Register>)) -> HttpResponse {
    debug!(
        "User registration attempt from: {:?} data:{:?}",
        req.connection_info().remote(),
        form.email
    );

    let accepted = match &form.completed {
        Some(thing) => thing.eq("on"),
        None => false,
    };

    if accepted {
        if let Ok(res) = new_user(&req, &form) {
            debug!("{:?}", res);
            return HttpResponse::Found()
                .header("location", "/user/login")
                .finish();
        }
    }
    HttpResponse::Found()
        .header("location", "/user/register")
        .finish()
}

pub fn hash_password(psw: String) -> Result<String, UserCreationError> {
    let salt = std::env::var("PW_SALT")?;
    let a = Argon2::new(3, 1, 4096, Variant::Argon2d)?;
    let mut out = [0 as u8; 128];
    a.hash(&mut out, psw.as_bytes(), salt.as_bytes(), &[], &[]);
    let psw_hash: String = out.iter().map(|b| format!("{:02x}", b)).collect();
    debug!("Pw hash: {:?}", psw_hash);
    Ok(psw_hash)
}

pub enum UserCreationError {
    MissingPassword,
    NotUnicodePassword,
    PasswordHashingError,
    UserExistError,
    WebError(actix_web::Error),
    DatabaseError(diesel::result::Error),
    MailBoxError(actix::MailboxError),
    UnknownError,
    MissingInvite,
    BadInvite,
}
impl From<std::env::VarError> for UserCreationError {
    fn from(error: std::env::VarError) -> Self {
        match error {
            std::env::VarError::NotPresent => UserCreationError::MissingPassword,
            std::env::VarError::NotUnicode(_) => UserCreationError::NotUnicodePassword,
        }
    }
}
impl From<argon2rs::ParamErr> for UserCreationError {
    fn from(_error: argon2rs::ParamErr) -> Self {
        UserCreationError::PasswordHashingError
    }
}
impl From<actix_web::Error> for UserCreationError {
    fn from(error: actix_web::Error) -> Self {
        UserCreationError::WebError(error)
    }
}
impl From<diesel::result::Error> for UserCreationError {
    fn from(error: diesel::result::Error) -> Self {
        UserCreationError::DatabaseError(error)
    }
}
impl From<actix::MailboxError> for UserCreationError {
    fn from(error: actix::MailboxError) -> Self {
        UserCreationError::MailBoxError(error)
    }
}

pub fn new_user(
    req: &HttpRequest<AppState>,
    form: &Register,
) -> Result<UserMeta, UserCreationError> {
    if let Ok(inv_expected) = std::env::var("INVITE_CODE") {
        if form.invite.is_none() {
            return Err(UserCreationError::MissingInvite);
        }
        if let Some(inv) = &form.invite {
            if inv_expected.ne(inv) {
                return Err(UserCreationError::BadInvite);
            }
        }
    }
    let usr_meta = UserMeta::load(req, form.email.clone());
    if usr_meta.is_ok() {
        error!("Attempted registration of existing User {:?}", req);
        return Err(UserCreationError::UserExistError);
    }

    let psw_hash = hash_password(form.psw.clone())?;
    debug!("Pw hash: {:?}", psw_hash);
    let usr_uuid = Uuid::new_v4();
    let nick = if form.display.is_none() {
        form.fname.clone() + ", " + form.lname.clone().as_str()
    } else {
        form.display.clone().unwrap()
    };

    let query = diesel::insert_into(users).values(uuid.eq(usr_uuid));
    let ins = WQuery {
        query,
        phantom: PhantomData::<User>,
    };
    let usrs = req.state().wdb.send(ins).wait()??;
    debug!("{:?}", usrs);
    let usr_id = usrs.first().unwrap().id;

    let query = diesel::insert_into(user_pwd).values((
        crate::schema::user_pwd::dsl::user_id.eq(usr_id),
        pw_hash.eq(psw_hash),
    ));
    let ins = WQuery {
        query,
        phantom: PhantomData::<UserPwd>,
    };
    let res = req.state().wdb.send(ins).wait()??;
    debug!("{:?}", res);

    let query = diesel::insert_into(user_meta).values((
        crate::schema::user_meta::dsl::user_id.eq(usr_id),
        fname.eq(form.fname.clone()),
        lname.eq(form.lname.clone()),
        email.eq(form.email.clone()),
        phone.eq(form.phone.clone()),
        display.eq(nick),
    ));
    let ins = WQuery {
        query,
        phantom: PhantomData::<UserMeta>,
    };
    let res = req.state().wdb.send(ins).wait()??;
    let ret = res.first().unwrap().clone();
    Ok(ret)
}

fn index_render() -> Result<String, Failure> {
    let mut links = crate::modules::navigation::default_menu();
    let register = Link::new("Login", "/user/login");
    links.push(register);
    let list = if std::env::var("INVITE_CODE").is_ok() {
        ructe_block_res!(crate::templates::user::register, true)?
    } else {
        ructe_block_res!(crate::templates::user::register, false)?
    };
    let meta = crate::modules::meta::default_meta("Register a new account");
    ructe_page_res!(crate::templates::navigation::empty_frame, meta, &list)
}

pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    http_ok(index_render())
}
