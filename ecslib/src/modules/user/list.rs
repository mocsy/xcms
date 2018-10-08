use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, SQuery};
use crate::modules::navigation::{Link, ListContext, Listing, Permission};
use crate::modules::user::UserMeta;
use crate::render::Failure;
use crate::schema::user_meta::dsl::*;
use crate::utils::http_ok;

fn index_render(data: Vec<Listing>) -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = crate::menu::default_menu();
    let ctx = ListContext {
        title: "User".to_string(),
        head: "List of users".to_string(),
        search: false,
    };
    let perm = crate::modules::navigation::PermissionSet {
        browse: true,
        read: true,
        edit: true,
        add: true,
        delete: true,
    };
    let list = ructe_block_res!(crate::templates::navigation::list, &data, &ctx, &perm)?;
    let meta = crate::modules::meta::default_meta("List of users");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    // let query = user_meta.filter(user_id.ne(-1i32));
    let query = user_meta.filter(user_id.is_not_null());
    let select = SQuery {
        select: query,
        phantom: PhantomData::<UserMeta>,
    };
    let usr_metas = req
        .state()
        .rdb
        .send(select)
        .map_err(actix_web::Error::from)
        .wait()
        .ok()
        .unwrap()
        .unwrap();
    let usr_list = create_listing(&usr_metas);
    http_ok(index_render(usr_list))
}

fn create_listing(metas: &[UserMeta]) -> Vec<Listing> {
    let mut data = Vec::new();
    for usr_meta in metas {
        let ed = Link {
            visual: "Edit".to_string(),
            url: format!("/user/{}", usr_meta.user_id),
            active: false,
            icon: "fa-edit".to_string(),
            clearance: Permission::Edit,
            children: None,
        };
        let del = Link {
            visual: "Delete".to_string(),
            url: format!("/user/{}", usr_meta.user_id),
            active: false,
            icon: "fa-trash".to_string(),
            clearance: Permission::Delete,
            children: None,
        };
        let usr_ls = Listing {
            id: usr_meta.user_id,
            name: usr_meta.display.clone(),
            date: usr_meta.created_at.to_string(),
            detail_first: format!("{}, {}", usr_meta.lname, usr_meta.fname),
            detail_last: usr_meta.email.clone(),
            comment: usr_meta.frozen.clone(),
            edit: ed,
            delete: del,
        };
        data.push(usr_ls);
    }
    data
}
