use crate::schema::access_control;
use chrono::{DateTime, Utc};
#[derive(Insertable, AsChangeset, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[table_name = "access_control"]
pub struct AccessControl {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub frozen: Option<String>,
    pub draft: Option<String>,
    pub last_update: DateTime<Utc>,
    pub updated_by: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessControlData {
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub frozen: Option<String>,
    pub draft: Option<String>,
    pub last_update: DateTime<Utc>,
    pub updated_by: String,
}
