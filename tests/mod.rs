#[cfg(test)]

mod tests {
    use super::*;
    use diesel::prelude::*;

    #[test]
    fn test_project() {
        let conn = db_connection();
        //       let query = todo.filter(id.ne(0));
        // let results = query
        //     .load::<Todo>(&conn)
        //     .expect("Error loading posts");
    }

    // fn normal_a() {
    //     use crate::schema::session_tokens::dsl::*;
    //     use crate::modules::user::token::TokenIdentity;
    //     use futures::future::Future;
    //     use futures::future::join_all;
    //     use actix_web::AsyncResponder;
    //     use actix_web::middleware;
    //     use actix_web::middleware::identity::IdentityService;
    //     use crate::modules::user::token::TokenIdentityPolicy;

    //     let query = session_tokens.filter(token.eq("88"))
    //         .limit(5);
    //     // let results = query
    //     //     .load::<TokenIdentity>(&conn)
    //     //     .expect("Error loading posts");

    //     // let res = crate::db::read(query, &conn);

    //     let addr = crate::db::db_setup();

    //     // let fut_result : Vec<Box<Request<DbExecutor<PgConnection>, crate::db::Queries<_,_>>>> = vec![
    //     //     Box::new( addr.send(crate::db::Queries::Select(query)) ),
    //     // ];
    //     let fut_result= vec![
    //         Box::new( addr.send(crate::db::Queries::Select(query)) ),
    //         Box::new( addr.send(crate::db::Queries::Any(query)) ),
    //         Box::new( addr.send(crate::db::Queries::Select(query)) ),
    //     ];
    //     // let fut_result= vec![
    //     //     Box::new( addr.send(crate::db::SQuery{select: query}) ),
    //     // ];
    //     let results = join_all(fut_result)
    //     .map_err(actix_web::Error::from)
    //     .and_then(|result| {
    //         println!("haho");
    //         println!("{:?}", result);
    //         let res: Vec<Option<Vec<TokenIdentity>>> =
    //             result.into_iter().map(|x| x.ok()).collect();
    //         assert!(res.len() ==0);
    //         println!("{:?}", res);
    //         Ok(HttpResponse::Ok().json(res))
    //         // Ok(HttpResponse::Ok())
    //     }).responder();
    // }

    pub fn db_connection() -> PgConnection {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    // #![feature(core_intrinsics)]
    // pub fn print_type_of<T>(_: &T) {
    //     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
    // }

}
