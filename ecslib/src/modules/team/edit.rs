use actix_web::{Error, Form, FromRequest, HttpRequest, HttpResponse, Path};

use diesel::prelude::*;
use futures::future::Future;
use std::marker::PhantomData;

use crate::db::{AppState, SQuery, WQuery};
use crate::modules::navigation::{EditableField, InputType, ListContext};
use crate::render::Failure;
use crate::utils::http_ok;

use super::data::{Team, TeamData};
use crate::modules::meta::default_meta;
use crate::schema::teams::dsl::*;

fn create_fields(data: &[Team]) -> Vec<EditableField> {
    let mut fields = Vec::new();
    let links = Vec::new();
    for typ in data {
        // let user_id_field = EditableField{
        //     input_type: InputType::Input,
        //     title: "User Id".to_string(),
        //     name: "user_id".to_string(),
        //     value: typ.user_id.to_string(),
        //     links: links.clone(),
        // };
        // fields.push(user_id_field);
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
        //     value: typ.frozen.clone().unwrap_or(String::new()),
        //     links: links.clone(),
        // };
        // fields.push(frozen_field);
        // let created_at_field = EditableField{
        //     input_type: InputType::Input,
        //     title: "Created At".to_string(),
        //     name: "created_at".to_string(),
        //     value: typ.created_at.to_string(),
        //     links: links.clone(),
        // };
        // fields.push(created_at_field);
    }
    fields
}
pub fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let org = Path::<String>::extract(req)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    debug!("{}", org);
    let query = teams.filter(id.eq(org));
    let select = SQuery {
        select: query,
        phantom: PhantomData::<Team>,
    };
    if let Ok(thing) = req
        .state()
        .rdb
        .send(select)
        .map_err(actix_web::Error::from)
        .wait()
    {
        if let Ok(data) = thing {
            let fields = create_fields(&data);
            return http_ok(index_render(fields));
        }
    }
    Ok(HttpResponse::Ok().finish())
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
    let org = Path::<String>::extract(&req)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    debug!("{}", org);
    //if id == form.id {
    let target = teams.filter(id.eq(org));
    let query = diesel::update(target).set((
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
    //}
    HttpResponse::Ok().finish()
}
