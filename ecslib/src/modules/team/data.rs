use crate::modules::user::User;
use crate::schema::teams;

#[derive(Insertable, AsChangeset, Queryable, Associations, Debug, Serialize, Deserialize, Clone)]
#[belongs_to(User)]
#[table_name = "teams"]
#[primary_key("id")]
pub struct Team {
    pub id: i64,
    pub access_control_id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub billing_name: String,
    pub billing_address: String,
    pub billing_city: String,
    pub billing_country: String,
    pub billing_zip: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamData {
    // pub access_control_id: ::uuid::Uuid,
    // pub user_id: i64,
    pub title: String,
    pub content: String,
    pub billing_name: String,
    pub billing_address: String,
    pub billing_city: String,
    pub billing_country: String,
    pub billing_zip: String,
}
