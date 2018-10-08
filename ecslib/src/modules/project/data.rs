use crate::schema::projects;
use chrono::{DateTime, Utc};

#[derive(
    Insertable,
    AsChangeset,
    Queryable,
    Associations,
    Identifiable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
#[table_name = "projects"]
#[primary_key("uuid")]
pub struct Project {
    pub projectid: i64,
    pub team_id: i64,
    pub uuid: ::uuid::Uuid,
    pub title: String,
    pub content: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProjectData {
    // pub team_id: i64,
    #[serde(default)]
    pub uuid: ::uuid::Uuid,
    pub title: String,
    pub content: Option<String>,
    #[serde(default)]
    pub ecs_start_date: Option<String>,
    #[serde(default)]
    pub ecs_end_date: Option<String>,
}
