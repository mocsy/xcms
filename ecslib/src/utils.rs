#[macro_use]
macro_rules! http_ok {
    ($html:expr) => {
        Ok(
            actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
                .content_type("text/html; charset=utf-8")
                .body($html),
        )
    };
}

pub fn http_ok(
    res: Result<String, crate::render::Failure>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    match res {
        Ok(s) => http_ok!(s),
        Err(_) => http_ok!("Render Failure"),
    }
}
