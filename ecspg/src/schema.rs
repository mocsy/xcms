table! {
    access_control (id) {
        id -> Int8,
        created_at -> Timestamptz,
        created_by -> Text,
        frozen -> Nullable<Text>,
        draft -> Nullable<Text>,
        last_update -> Timestamptz,
        updated_by -> Text,
    }
}

table! {
    access_group_members (id) {
        id -> Int8,
        access_group_id -> Int8,
        user_id -> Int8,
        access_control_id -> Int8,
    }
}

table! {
    access_groups (id) {
        id -> Int8,
        name -> Text,
        tag -> Nullable<Jsonb>,
        access_control_id -> Int8,
    }
}

table! {
    access_keys (id) {
        id -> Int8,
        key -> Text,
        access_type -> Text,
        user_id -> Int8,
        reason -> Text,
        expiry -> Timestamptz,
        tag -> Nullable<Jsonb>,
        access_control_id -> Int8,
    }
}

table! {
    access_rules (id) {
        id -> Int8,
        access_group_id -> Int8,
        access_control_id -> Int8,
        access_type -> Text,
    }
}

table! {
    api_keys (id) {
        id -> Int8,
        team_id -> Int8,
        api_key -> Text,
        access_control_id -> Int8,
    }
}

table! {
    menus (id) {
        id -> Int8,
        title -> Text,
        links -> Nullable<Jsonb>,
        tag -> Nullable<Jsonb>,
        access_control_id -> Int8,
    }
}

table! {
    organizers (id) {
        id -> Int8,
        access_control_id -> Int8,
        user_id -> Int8,
        title -> Text,
        content -> Text,
        billing_name -> Text,
        billing_address -> Text,
        billing_city -> Text,
        billing_country -> Text,
        billing_zip -> Text,
    }
}

table! {
    projects (uuid) {
        projectid -> Int8,
        team_id -> Int8,
        uuid -> Uuid,
        title -> Text,
        content -> Text,
        start_date -> Nullable<Timestamptz>,
        end_date -> Nullable<Timestamptz>,
    }
}

table! {
    session_tokens (token) {
        token -> Text,
        claim -> Varchar,
    }
}

table! {
    teams (id) {
        id -> Int8,
        access_control_id -> Int8,
        user_id -> Int8,
        title -> Text,
        content -> Text,
        billing_name -> Text,
        billing_address -> Text,
        billing_city -> Text,
        billing_country -> Text,
        billing_zip -> Text,
    }
}

table! {
    todos (id) {
        id -> Int8,
        title -> Text,
        description -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        project_id -> Uuid,
        completed -> Bool,
        completed_at -> Timestamptz,
    }
}

table! {
    user_meta (email) {
        user_id -> Int8,
        display -> Text,
        fname -> Text,
        lname -> Text,
        email -> Text,
        phone -> Text,
        frozen -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

table! {
    user_pwd (id) {
        id -> Int8,
        user_id -> Int8,
        pw_hash -> Text,
        frozen -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        uuid -> Uuid,
    }
}

joinable!(access_group_members -> access_control (access_control_id));
joinable!(access_group_members -> access_groups (access_group_id));
joinable!(access_group_members -> users (user_id));
joinable!(access_groups -> access_control (access_control_id));
joinable!(access_keys -> access_control (access_control_id));
joinable!(access_keys -> users (user_id));
joinable!(access_rules -> access_control (access_control_id));
joinable!(access_rules -> access_groups (access_group_id));
joinable!(api_keys -> access_control (access_control_id));
joinable!(api_keys -> teams (team_id));
joinable!(menus -> access_control (access_control_id));
joinable!(organizers -> access_control (access_control_id));
joinable!(organizers -> users (user_id));
joinable!(teams -> access_control (access_control_id));
joinable!(teams -> users (user_id));
joinable!(todos -> projects (project_id));
joinable!(user_meta -> users (user_id));
joinable!(user_pwd -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_control,
    access_group_members,
    access_groups,
    access_keys,
    access_rules,
    api_keys,
    menus,
    organizers,
    projects,
    session_tokens,
    teams,
    todos,
    user_meta,
    user_pwd,
    users,
);
