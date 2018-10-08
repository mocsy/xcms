use actix_web::{Error, HttpRequest, HttpResponse};

use crate::db::AppState;
use crate::modules::navigation::{Link, ListContext, Listing, Permission};
use crate::render::Failure;
use crate::utils::http_ok;

fn index_render() -> Result<String, Failure> {
    let toplinks = Vec::new();
    let links = crate::modules::navigation::default_menu();
    let ed = Link {
        visual: "Edit".to_string(),
        url: "/user/list".to_string(),
        active: false,
        icon: "fa-edit".to_string(),
        clearance: Permission::Edit,
        children: None,
    };
    let del = Link {
        visual: "Delete".to_string(),
        url: "/user/list".to_string(),
        active: false,
        icon: "fa-trash".to_string(),
        clearance: Permission::Delete,
        children: None,
    };
    let data = vec![Listing {
        id: 0,
        name: "George Stoneheart".to_string(),
        date: "2018-10-08".to_string(),
        detail_first: "Nice and big Project 2018".to_string(),
        detail_last: "Free of charge".to_string(),
        comment: None,
        edit: ed,
        delete: del,
    }];
    let ctx = ListContext {
        title: "List".to_string(),
        head: "List of things".to_string(),
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
    let meta = crate::modules::meta::default_meta("List");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    http_ok(index_render())
}
