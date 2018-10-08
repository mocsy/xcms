use actix_web::middleware::identity::RequestIdentity;
use actix_web::{Error, Form, HttpRequest, HttpResponse};

use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, WQuery};
use crate::modules::navigation::{EditableField, InputType, ListContext};
use crate::modules::user::UserMeta;
use crate::render::Failure;
use crate::utils::http_ok;

use super::data::{Team, TeamData};
use crate::modules::access::*;
use crate::modules::meta::default_meta;
use crate::schema::api_keys::dsl::*;
use crate::schema::teams::dsl::*;

fn create_fields() -> Vec<EditableField> {
    let mut fields = Vec::new();
    let links = Vec::new();
    let typ = Team {
        id: 0,
        access_control_id: 0,
        user_id: 0,
        title: String::new(),
        content: String::new(),
        billing_name: String::new(),
        billing_address: String::new(),
        billing_city: String::new(),
        billing_country: String::new(),
        billing_zip: String::new(),
        // frozen: Some(String::new()),
        // created_at: Utc::now(),
    };
    // let user_id_field = EditableField{
    //     input_type: InputType::Input,
    //     title: "User Id".to_string(),
    //     name: "user_id".to_string(),
    //     value: typ.user_id.to_string(),
    //     links: links.clone(),
    //     };
    //     fields.push(user_id_field);
    let title_field = EditableField {
        input_type: InputType::Input,
        title: "Title".to_string(),
        name: "title".to_string(),
        value: typ.title.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(title_field);
    let content_field = EditableField {
        input_type: InputType::TextArea,
        title: "Content".to_string(),
        name: "content".to_string(),
        value: typ.content.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(content_field);
    let billing_name_field = EditableField {
        input_type: InputType::Input,
        title: "Billing Name".to_string(),
        name: "billing_name".to_string(),
        value: typ.billing_name.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(billing_name_field);
    let billing_address_field = EditableField {
        input_type: InputType::Input,
        title: "Billing Address".to_string(),
        name: "billing_address".to_string(),
        value: typ.billing_address.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(billing_address_field);
    let billing_city_field = EditableField {
        input_type: InputType::Input,
        title: "Billing City".to_string(),
        name: "billing_city".to_string(),
        value: typ.billing_city.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(billing_city_field);
    let billing_country_field = EditableField {
        input_type: InputType::Input,
        title: "Billing Country".to_string(),
        name: "billing_country".to_string(),
        value: typ.billing_country.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(billing_country_field);
    let billing_zip_field = EditableField {
        input_type: InputType::Input,
        title: "Billing Zip".to_string(),
        name: "billing_zip".to_string(),
        value: typ.billing_zip.to_string(),
        links: links.clone(),
        required: false,
    };
    fields.push(billing_zip_field);
    // let frozen_field = EditableField{
    //     input_type: InputType::Input,
    //     title: "Frozen".to_string(),
    //     name: "frozen".to_string(),
    //     value: String::new(),
    //     links: links.clone(),
    //     };
    //     fields.push(frozen_field);
    // let created_at_field = EditableField{
    //     input_type: InputType::Input,
    //     title: "Created At".to_string(),
    //     name: "created_at".to_string(),
    //     value: typ.created_at.to_string(),
    //     links: links.clone(),
    //     };
    //     fields.push(created_at_field);
    fields
}
pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    // let id = 0i32;
    let fields = create_fields();
    http_ok(index_render(fields))
}
fn index_render(fields: Vec<EditableField>) -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = crate::menu::default_menu();
    let ctx = ListContext {
        title: "Team".to_string(),
        head: "Team Editor".to_string(),
        search: false,
    };
    let perm = crate::modules::navigation::PermissionSet {
        browse: true,
        read: true,
        edit: true,
        add: true,
        delete: true,
    };
    let list = ructe_block_res!(crate::templates::navigation::edit, &fields, &ctx, &perm)?;
    let meta = default_meta("Team Editor");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn save((req, form): (HttpRequest<AppState>, Form<TeamData>)) -> HttpResponse {
    // let id = 0i32;
    if let Some(mail) = req.identity() {
        if let Ok(usr_meta) = UserMeta::load(&req, mail) {
            let access_res = access_control_entry(&req);
            if let Ok(access) = access_res {
                use diesel::insert_into;
                let query = insert_into(teams).values((
                    crate::schema::teams::access_control_id.eq(access.id),
                    crate::schema::teams::user_id.eq(usr_meta.user_id),
                    title.eq(form.title.clone()),
                    content.eq(form.content.clone()),
                    billing_name.eq(form.billing_name.clone()),
                    billing_address.eq(form.billing_address.clone()),
                    billing_city.eq(form.billing_city.clone()),
                    billing_country.eq(form.billing_country.clone()),
                    billing_zip.eq(form.billing_zip.clone()),
                ));
                let upd = WQuery {
                    query,
                    phantom: PhantomData::<Team>,
                };
                let res = req
                    .state()
                    .wdb
                    .send(upd)
                    .map_err(actix_web::Error::from)
                    .wait()
                    .ok()
                    .unwrap()
                    .unwrap();
                debug!("{:?}", res);
                let org = res.first().unwrap();
                let key = format!("{:X}", rand::random::<u128>());
                let query = insert_into(api_keys).values((
                    team_id.eq(org.id),
                    api_key.eq(key),
                    crate::schema::api_keys::access_control_id.eq(access.id),
                ));
                let upd = WQuery {
                    query,
                    phantom: PhantomData::<(i64, i64, String, i64)>,
                };
                let res = req.state().wdb.send(upd).wait().unwrap().unwrap();
                debug!("{:?}", res);
            }
        }
    }
    HttpResponse::Found().header("location", "list").finish()
}
