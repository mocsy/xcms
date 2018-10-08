use crate::schema::access_rules;
#[derive(Insertable, AsChangeset, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[table_name = "access_rules"]
pub struct AccessRule {
    pub id: i64,
    pub access_group_id: i64,
    pub access_control_id: i64,
    pub access_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessRuleData {
    pub access_type: String,
}
