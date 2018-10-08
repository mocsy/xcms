use actix_web::middleware::session::RequestSession;
use actix_web::{Error, HttpRequest, HttpResponse};

use crate::db::AppState;
use crate::render::Failure;
use crate::utils::http_ok;

use crate::menu::default_menu;
use crate::modules::meta::default_meta;

pub fn select(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    debug!("{:?}", req);
    let _res = req.session().set("org", 1);
    http_ok(select_render())
}

fn select_render() -> Result<String, Failure> {
    debug!("zazaza");
    let toplinks = crate::menu::default_top_menu();
    let meta = default_meta("Select Team");
    let links = default_menu();
    let cnt = ructe_block_res!(crate::templates::team::selector)?;
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &cnt
    )
}
