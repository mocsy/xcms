use actix_web::http::header;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::middleware::{Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse, Result};

pub struct Restrict;
impl<S> Middleware<S> for Restrict {
    /// Method is called when request is ready. It may return
    /// future, which should resolve before next middleware get called.
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if req.identity().is_some() {
            return Ok(Started::Done);
        }

        if req.path() == "/user/login" || req.path() == "/user/register" || req.path() == "/static"
        {
            return Ok(Started::Done);
        }

        //Redirect to login if not logged in
        Ok(Started::Response(
            HttpResponse::Found()
                .header(header::LOCATION, "/user/login")
                .finish(),
        ))
    }

    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        if req.cookie("redalfrom").is_some() && req.identity().is_some() {
            resp.del_cookie("redalfrom");
        }
        // if req.cookie("redalfrom").is_none() && req.identity().is_none() {
        //     // println!("Path: {:?}", req.path());

        //     let s =
        //     //let s = if req.path() == "/user/login" || req.path() == "/user/register" {
        //     //    "/user/dashboard".to_owned()
        //     //} else {
        //         if resp.status().eq(&StatusCode::NOT_FOUND) {
        //             "/".to_owned()
        //         } else {
        //             req.path().to_owned()
        //         };
        //     //};
        //     if req.path() != "/user/logout" && req.path() != "/user/login" && req.path() != "/user/register" {
        //         let ck: Cookie = Cookie::build("redalfrom", Cow::from(s))
        //             .path("/user/login")
        //             .secure(true)
        //             .http_only(true)
        //             .same_site(SameSite::Strict)
        //             .max_age(Duration::minutes(10))
        //             .finish();
        //         resp.add_cookie(&ck).unwrap();
        //     }
        // }
        Ok(Response::Done(resp))
    }
}
