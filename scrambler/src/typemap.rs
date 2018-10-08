// generated from https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html
pub fn get_rust_type_str(sql_type: String, udt_name: String, nullable: bool) -> String {
    if sql_type.eq(&udt_name) {
        let tp = get_rust_type(sql_type);
        if nullable {
            format!("Option<{}>", tp)
        } else {
            tp.to_owned()
        }
    } else {
        let stype = udt_name.trim_start_matches('_').to_owned();
        let rtype = get_rust_type(stype);
        match sql_type.as_ref() {
            "ARRAY" => format!("Vec<{}>", rtype),
            "bigint" => "i64".to_owned(),
            "smallint" => "i16".to_owned(),
            "timestamp with time zone" => "DateTime<Utc>".to_owned(),
            "boolean" => "bool".to_owned(),
            _ => panic!("mapping for {},{} is not defined", sql_type, udt_name),
        }
        // if "ARRAY".eq(&sql_type) {
        //     format!("Vec<{}>", rtype)
        // } else {
        //     panic!("mapping for {},{} is not defined", sql_type, udt_name);
        // }
    }
}

fn get_rust_type(sql_type: String) -> &'static str {
    match sql_type.as_ref() {
        "smallint" => "i16",
        "int2" => "i16",
        "int" => "i32",
        "int4" => "i32",
        "bigint" => "i64",
        "int8" => "i64",
        "numeric(p, s)" => "bigdecimal::BigDecimal",
        "decimal(p, s)" => "bigdecimal::BigDecimal",
        "real" => "f32",
        "float4" => "f32",
        "double precision" => "f64",
        "float8" => "f64",
        "smallserial" => "i16",
        "serial2" => "i16",
        "serial" => "i32",
        "serial4" => "i32",
        "bigserial" => "i64",
        "serial8" => "i64",
        //"Monetary Types"=>"",
        "money" => "Cents",
        //"Character Types"=>"",
        "character varying(n)" => "String",
        "varchar(n)" => "String",
        "character(n)" => "String",
        "char(n)" => "String",
        "text" => "String",
        //"Binary Data Types"=>"",
        "bytea" => "Vec<u8>",
        //"Date/Time Types"=>"",
        "timestamp" => "chrono::NaiveDateTime",
        "timestamp(p)" => "chrono::NaiveDateTime",
        "date" => "chrono::NaiveDate",
        "time" => "chrono::NaiveTime",
        "time(p)" => "chrono::NaiveTime",
        //"Boolean Type"=>"",
        "boolean" => "bool",
        "bool" => "bool",
        "cidr" => "ipnetwork::IpNetwork",
        "inet" => "ipnetwork::IpNetwork",
        "macaddr" => "[u8; 6]",
        "enum" => "String",
        "uuid" => "::uuid::Uuid",
        //"JSON Types"=>"",
        "json" => "serde_json::Value",
        "jsonb" => "serde_json::Value",
        //"Range Types"=>"",
        "int4range" => "(Bound<i32>, Bound<i32>)",
        "int8range" => "(Bound<i64>, Bound<i64>)",
        "numrange" => "(Bound<bigdecimal::BigDecimal>,Bound<bigdecimal::BigDecimal>)",
        "tsrange" => "(Bound<chrono::NaiveDateTime>,Bound<chrono::NaiveDateTime>)",
        "tstzrange" => "(Bound<chrono::DateTime>, Bound<chrono::DateTime>)",
        "daterange" => "(Bound<chrono::NaiveDate>, Bound<chrono::NaiveDate>)",

        //Could not figure out, use String
        _ => "String",
    }
}
