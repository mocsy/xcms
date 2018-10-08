use heck::CamelCase;

pub fn type_name(table_name: String) -> String {
    if table_name.ends_with('s') {
        let mut res = table_name.to_camel_case();
        res.pop();
        return res;
    }
    table_name.to_camel_case()
}
