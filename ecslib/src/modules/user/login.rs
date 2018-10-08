use actix_web::middleware::identity::RequestIdentity;
use actix_web::{Error, Form, HttpRequest, HttpResponse};
use argon2rs::{Argon2, Variant};
use diesel::prelude::*;
use futures::future::Future;
use log::{debug, info};
use std::marker::PhantomData;

use crate::db::{AppState, SQuery};
use crate::render::Failure;
use crate::utils::http_ok;
// use crate::schema::users::dsl::*;
// use crate::schema::user_meta::dsl::*;
use crate::modules::navigation::Link;
use crate::modules::user::{UserMeta, UserPwd};
use crate::schema::user_pwd::dsl::*;

// TODO: remove once not needed
use actix_web::middleware::session::RequestSession;

fn index_render() -> Result<String, Failure> {
    let mut links = crate::modules::navigation::default_menu();
    let register = Link::new("Register", "/user/register");
    links.push(register);
    let login = ructe_block_res!(crate::templates::user::login, "User login")?;
    let meta = crate::modules::meta::default_meta("Login to the application");
    ructe_page_res!(crate::templates::navigation::empty_frame, meta, &login)
}
pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    http_ok(index_render())
}

#[derive(Deserialize)]
pub struct LoginParams {
    email: String,
    psw: String,
    remember: Option<String>,
}
// pub fn login(form: Form<LoginParams>) -> WebResult<String> {
//     Ok(format!("Welcome {}!", form.uname))
// }
pub fn login((req, form): (HttpRequest<AppState>, Form<LoginParams>)) -> HttpResponse {
    if let Ok(salt) = std::env::var("PW_SALT") {
        let mut after_login = String::from("/user/list");
        if let Some(cookie) = req.cookie("redalfrom") {
            after_login = cookie.value().to_owned();
        }
        debug!("Login to: {}", after_login.clone());
        let usr_meta_result = UserMeta::load(&req, form.email.clone());
        debug!(
            "Login request from: {:?},{:?} {:?} id:{:?}",
            req.connection_info().remote(),
            form.email.clone(),
            form.remember,
            usr_meta_result
        );
        if let Ok(usr_meta) = usr_meta_result {
            let usr_id = usr_meta.user_id;

            let a = Argon2::new(3, 1, 4096, Variant::Argon2d).ok().unwrap();
            let mut out = [0 as u8; 128];
            a.hash(&mut out, form.psw.as_bytes(), salt.as_bytes(), &[], &[]);
            let calc_hash: String = out.iter().map(|b| format!("{:02x}", b)).collect();

            let query = user_pwd.filter(crate::schema::user_pwd::dsl::user_id.eq(usr_id));
            let select = SQuery {
                select: query,
                phantom: PhantomData::<UserPwd>,
            };
            let usr_pwds = req
                .state()
                .rdb
                .send(select)
                .map_err(actix_web::Error::from)
                .wait()
                .ok()
                .unwrap()
                .unwrap();
            let stored_hash = usr_pwds.first().unwrap().pw_hash.clone();
            debug!("stored_hash: {}", stored_hash.clone());
            if calc_hash == stored_hash {
                req.remember(form.email.clone());

                // TODO: remove once not needed
                let _res = req.session().set("org", 1);
                info!("Login successfull {}", form.email);

                return HttpResponse::Found()
                    .header("location", after_login)
                    .finish();
            } else {
                info!("Login wrong password {}", form.email);
            }
        } else {
            info!("Failed login {:?} {}", usr_meta_result, form.email);
            // return HttpResponse::Found().header("location", "/user/register").finish()
        }
    }
    HttpResponse::Found()
        .header("location", "/user/login")
        .finish()
}
pub fn logout(req: &HttpRequest<AppState>) -> HttpResponse {
    debug!("Handling logout request: {:?}", req);
    req.forget();
    HttpResponse::Found()
        .header("location", "/user/login")
        .finish()
}
