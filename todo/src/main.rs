#[macro_use]
extern crate log;

use actix_diesel_actor as db;
use actix_web::http::{header, Method, NormalizePath};
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::session::{CookieSessionBackend, SessionStorage};
use actix_web::{fs, middleware, server, App, HttpResponse};
use pretty_env_logger;

use ecslib::modules;

use crate::modules::user::restrict::Restrict;

fn main() {
    std::fs::create_dir_all("static").unwrap_or_else(|e| panic!("{}", e));
    dotenv::dotenv().ok();
    ::std::env::set_var(
        "RUST_LOG",
        "actix=debug,actix_web=debug,debug,tokio=info,h2=info,rustls=info,actix_diesel_actor=debug",
    );
    // ::std::env::set_var("RUST_BACKTRACE", "0");
    let bind_url =
        std::env::var("BIND_URL_CH").unwrap_or_else(|_| panic!("{} must be set", "BIND_URL_CH"));
    pretty_env_logger::init();
    let sys = actix::System::new("ecs_actors");

    #[cfg(feature = "https")]
    let secure = true;
    #[cfg(not(feature = "https"))]
    let secure = false;

    let raddr = crate::db::db_setup(crate::db::ConnectionType::Read);
    let waddr = crate::db::db_setup(crate::db::ConnectionType::Write);
    // routes need to be defined in a most specific to least specific order
    let srv = server::new(move || {
        vec![
            App::with_state(crate::db::AppState {
                rdb: raddr.clone(),
                wdb: waddr.clone(),
            })
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 128])
                    .name("auth-cookie")
                    .secure(secure),
            ))
            .middleware(Restrict)
            .middleware(SessionStorage::new(
                CookieSessionBackend::private(&[0; 32]).secure(secure),
            ))
            .prefix("/project")
            .resource("list", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::list::index)
            })
            .resource("add", |r| {
                r.method(Method::GET).f(crate::modules::project::add::index);
                r.method(Method::POST)
                    .with(crate::modules::project::add::save);
            })
            .resource("{id}/todo", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::todo::index);
                r.method(Method::POST)
                    .with(crate::modules::project::todo::toggle);
            })
            .resource("{id}/edit", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::edit::index);
                r.method(Method::POST)
                    .with(crate::modules::project::edit::save);
            })
            .resource("{id}/register", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::todo_register::index);
                r.method(Method::POST)
                    .with(crate::modules::project::todo_register::save);
            })
            .resource("{id}/todo/{aid}", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::todo_register::edit_page);
                r.method(Method::POST)
                    .with(crate::modules::project::todo_register::save_todo);
            })
            .resource("{id}/todo/{aid}/delete", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::todo_register::delete_todo);
            })
            .resource("todolist", |r| {
                r.method(Method::GET)
                    .f(crate::modules::project::todo_list::index)
            })
            .resource("/", |r| {
                r.method(Method::GET).f(|_req| {
                    HttpResponse::Found()
                        .header(header::LOCATION, "/project/list")
                        .finish()
                })
            })
            .default_resource(|r| r.method(Method::GET).h(NormalizePath::default())),
            App::with_state(crate::db::AppState {
                rdb: raddr.clone(),
                wdb: waddr.clone(),
            })
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 128])
                    .name("auth-cookie")
                    .secure(secure),
            ))
            .middleware(Restrict)
            .middleware(SessionStorage::new(
                CookieSessionBackend::private(&[0; 32]).secure(secure),
            ))
            .prefix("/user")
            .resource("login", |r| {
                r.method(Method::GET).f(crate::modules::user::login::index);
                r.method(Method::POST)
                    .with(crate::modules::user::login::login);
            })
            .resource("register", |r| {
                r.method(Method::GET)
                    .f(crate::modules::user::register::index);
                r.method(Method::POST)
                    .with(crate::modules::user::register::save);
            })
            .resource("logout", |r| r.f(crate::modules::user::login::logout))
            .resource("list", |r| {
                r.method(Method::GET).f(|_req| {
                    HttpResponse::Found()
                        .header(header::LOCATION, "/project/todolist")
                        .finish()
                })
            })
            .default_resource(|r| r.method(Method::GET).h(NormalizePath::default())),
            App::with_state(crate::db::AppState {
                rdb: raddr.clone(),
                wdb: waddr.clone(),
            })
            .middleware(middleware::Logger::default())
            .prefix("/static")
            .handler("/", fs::StaticFiles::new("./static/").unwrap())
            .default_resource(|r| r.method(Method::GET).h(NormalizePath::default())),
            App::with_state(crate::db::AppState {
                rdb: raddr.clone(),
                wdb: waddr.clone(),
            })
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 128])
                    .name("auth-cookie")
                    .secure(secure),
            ))
            .middleware(Restrict)
            .middleware(SessionStorage::new(
                CookieSessionBackend::private(&[0; 32]).secure(secure),
            ))
            .resource("/", |r| {
                r.method(Method::GET).f(|_req| {
                    HttpResponse::Found()
                        .header(header::LOCATION, "/project/list")
                        .finish()
                })
            }),
        ]
    });

    #[cfg(feature = "https")]
    {
        use actix_web::server::ServerFlags;
        use rustls::internal::pemfile::{certs, pkcs8_private_keys};
        use rustls::{NoClientAuth, ServerConfig};
        use std::fs::File;
        use std::io::BufReader;
        // load ssl keys
        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(
            File::open("cert.pem").unwrap_or_else(|e| panic!("cert.pem {}", e)),
        );
        let key_file =
            &mut BufReader::new(File::open("key.pem").unwrap_or_else(|e| panic!("{}", e)));
        let cert_chain = certs(cert_file).unwrap_or_else(|e| panic!("{:#?}", e));
        // let mut keys = rsa_private_keys(key_file)
        //     .unwrap_or_else(|e| panic!("{:#?}", e));
        let mut keys = pkcs8_private_keys(key_file).unwrap_or_else(|e| panic!("{:#?}", e));
        config
            .set_single_cert(cert_chain, keys.remove(0))
            .unwrap_or_else(|e| panic!("{:#?}", e));

        // actix acceptor
        let acceptor =
            server::RustlsAcceptor::with_flags(config, ServerFlags::HTTP1 | ServerFlags::HTTP2);
        info!("Listening on https://{}", bind_url);
        srv.bind_with(bind_url, move || acceptor.clone())
            .unwrap()
            .run();
    }
    #[cfg(not(feature = "https"))]
    {
        info!("Listening on http://{}", bind_url);
        srv.bind(bind_url).unwrap().run();
    }
    let _ = sys.run();
}
