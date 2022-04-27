table! {
    action_record (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        user_id -> Integer,
        uri -> Varchar,
        match_value -> Varchar,
        controller_name -> Varchar,
        action_name -> Varchar,
        body -> Nullable<Longtext>,
        action -> Varchar,
        method -> Varchar,
        ret_code -> Integer,
        ret_body -> Nullable<Longtext>,
        request_detail -> Nullable<Longtext>,
        description -> Nullable<Longtext>,
    }
}

table! {
    config_version (id) {
        id -> Integer,
        version -> Integer,
        sub_version -> Varchar,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
    }
}

table! {
    deleted_data (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        origin_value -> Nullable<Text>,
        user_id -> Integer,
    }
}

table! {
    matcher (id) {
        id -> Bigint,
        match_type -> Integer,
        match_value -> Varchar,
        match_target -> Integer,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        upstream -> Varchar,
        match_method -> Varchar,
    }
}

table! {
    matcher_module (id) {
        id -> Bigint,
        matcher_id -> Bigint,
        module_id -> Bigint,
        sort_index -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
    }
}

table! {
    matcher_module_snapshot (id) {
        id -> Bigint,
        matcher_id -> Bigint,
        module_id -> Bigint,
        sort_index -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
        snapshot_id -> Integer,
    }
}

table! {
    matcher_snapshot (id) {
        id -> Bigint,
        match_type -> Integer,
        match_value -> Varchar,
        match_target -> Integer,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        upstream -> Varchar,
        snapshot_id -> Integer,
        original_id -> Integer,
        match_method -> Varchar,
    }
}

table! {
    matcher_test (id) {
        id -> Bigint,
        match_type -> Integer,
        match_value -> Varchar,
        match_target -> Integer,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        upstream -> Varchar,
        match_method -> Varchar,
    }
}

table! {
    module (id) {
        id -> Bigint,
        module_name -> Varchar,
        module_desc -> Nullable<Varchar>,
        global_config_names -> Nullable<Text>,
        worker_config_names -> Nullable<Text>,
        request_config_names -> Nullable<Text>,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
        full_name -> Nullable<Varchar>,
        sort_index -> Integer,
    }
}

table! {
    module_config (id) {
        id -> Bigint,
        module_id -> Bigint,
        scope -> Varchar,
        config_name -> Varchar,
        config_value -> Nullable<Text>,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
        config_reason -> Nullable<Text>,
    }
}

table! {
    module_config_snapshot (id) {
        id -> Bigint,
        module_id -> Bigint,
        scope -> Varchar,
        config_name -> Varchar,
        config_value -> Nullable<Text>,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
        snapshot_id -> Integer,
        config_reason -> Nullable<Text>,
    }
}

table! {
    module_snapshot (id) {
        id -> Bigint,
        module_name -> Varchar,
        module_desc -> Nullable<Varchar>,
        global_config_names -> Nullable<Text>,
        worker_config_names -> Nullable<Text>,
        request_config_names -> Nullable<Text>,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
        full_name -> Nullable<Varchar>,
        sort_index -> Integer,
        snapshot_id -> Integer,
        original_id -> Integer,
    }
}

table! {
    ng_admin_user (id) {
        id -> Integer,
        user_name -> Varchar,
        pass_word -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        user_group -> Integer,
        user_role -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    ng_admin_user_group (id) {
        id -> Integer,
        group_name -> Varchar,
        group_father -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    ng_host (id) {
        id -> Integer,
        host_name -> Varchar,
        ip -> Varchar,
        status -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        version -> Varchar,
        config_version -> Varchar,
    }
}

table! {
    request_config (id) {
        id -> Bigint,
        matcher_id -> Bigint,
        module_id -> Bigint,
        config_name -> Varchar,
        config_value -> Nullable<Longtext>,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
    }
}

table! {
    request_config_snapshot (id) {
        id -> Bigint,
        matcher_id -> Bigint,
        module_id -> Bigint,
        config_name -> Varchar,
        config_value -> Nullable<Longtext>,
        version -> Integer,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
        snapshot_id -> Integer,
    }
}

table! {
    snapshot_info (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        snapshot_name -> Varchar,
        snapshot_desc -> Varchar,
        creator -> Integer,
    }
}

table! {
    upstream (id) {
        id -> Integer,
        #[sql_name = "upstream"]
        upstream_name -> Varchar,
        name -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        created_at -> Datetime,
    }
}

joinable!(matcher_module -> matcher (matcher_id));
joinable!(matcher_module -> module (module_id));
joinable!(module_config -> module (module_id));
joinable!(request_config -> matcher (matcher_id));
joinable!(request_config -> module (module_id));

allow_tables_to_appear_in_same_query!(
    action_record,
    config_version,
    deleted_data,
    matcher,
    matcher_module,
    matcher_module_snapshot,
    matcher_snapshot,
    matcher_test,
    module,
    module_config,
    module_config_snapshot,
    module_snapshot,
    ng_admin_user,
    ng_admin_user_group,
    ng_host,
    request_config,
    request_config_snapshot,
    snapshot_info,
    upstream,
);
