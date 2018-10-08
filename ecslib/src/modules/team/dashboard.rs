use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

use crate::db::AppState;
use crate::modules::meta::default_meta;
use crate::render::Failure;
use crate::utils::http_ok;

fn index_render() -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = crate::menu::default_menu();
    let mut list = ructe_block_res!(crate::templates::team::dashboard)?;
    list.push_str(r#"<script src="/static/dashboard.js" charset="utf-8"></script>"#);
    let meta = default_meta("Dashboard");
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
