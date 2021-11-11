table! {
    architecture (id) {
        id -> Int4,
        code -> Varchar,
    }
}

table! {
    build (id) {
        id -> Int4,
        version_id -> Int4,
        firmware_id -> Int4,
        publisher_user_id -> Nullable<Int4>,
        checksum -> Nullable<Varchar>,
        extract_size -> Nullable<Int4>,
        path -> Varchar,
        md5 -> Nullable<Varchar>,
        insert_date -> Timestamp,
        active -> Nullable<Bool>,
    }
}

table! {
    build_architecture (build_id, architecture_id) {
        build_id -> Int4,
        architecture_id -> Int4,
    }
}

table! {
    description (version_id, language_id) {
        version_id -> Int4,
        language_id -> Int4,
        #[sql_name = "description"]
        desc -> Text,
    }
}

table! {
    displayname (version_id, language_id) {
        version_id -> Int4,
        language_id -> Int4,
        #[sql_name = "displayname"]
        name -> Varchar,
    }
}

table! {
    download (id) {
        id -> Int4,
        build_id -> Int4,
        architecture_id -> Int4,
        firmware_build -> BigInt,
        ip_address -> Varchar,
        user_agent -> Nullable<Varchar>,
        date -> Timestamp,
    }
}

table! {
    firmware (id) {
        id -> Int4,
        version -> Varchar,
        build -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::IconSize;
    icon (id) {
        id -> Int4,
        version_id -> Int4,
        size -> IconSize,
        path -> Varchar,
    }
}

table! {
    language (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
    }
}

table! {
    package (id) {
        id -> Int4,
        author_user_id -> Nullable<Int4>,
        name -> Varchar,
        insert_date -> Nullable<Timestamp>,
    }
}

table! {
    package_user_maintainer (package_id, user_id) {
        package_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    role (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    screenshot (id) {
        id -> Int4,
        package_id -> Int4,
        path -> Varchar,
    }
}

table! {
    service (id) {
        id -> Int4,
        code -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        api_key -> Nullable<Varchar>,
        github_access_token -> Nullable<Varchar>,
        active -> Bool,
        confirmed_at -> Nullable<Timestamp>,
    }
}

table! {
    user_role (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    version (id) {
        id -> Int4,
        package_id -> Int4,
        #[sql_name = "version"]
        ver -> Int4,
        upstream_version -> Varchar,
        changelog -> Nullable<Text>,
        report_url -> Nullable<Varchar>,
        distributor -> Nullable<Varchar>,
        distributor_url -> Nullable<Varchar>,
        maintainer -> Nullable<Varchar>,
        maintainer_url -> Nullable<Varchar>,
        dependencies -> Nullable<Varchar>,
        conf_dependencies -> Nullable<Varchar>,
        conflicts -> Nullable<Varchar>,
        conf_conflicts -> Nullable<Varchar>,
        install_wizard -> Nullable<Bool>,
        upgrade_wizard -> Nullable<Bool>,
        startable -> Nullable<Bool>,
        license -> Nullable<Text>,
        insert_date -> Timestamp,
    }
}

table! {
    version_service_dependency (version_id, package_id) {
        version_id -> Int4,
        package_id -> Int4,
    }
}

joinable!(build -> firmware (firmware_id));
joinable!(build -> version (version_id));
joinable!(build -> user (publisher_user_id));
joinable!(build_architecture -> architecture (architecture_id));
joinable!(build_architecture -> build (build_id));
joinable!(description -> language (language_id));
joinable!(description -> version (version_id));
joinable!(displayname -> language (language_id));
joinable!(displayname -> version (version_id));
joinable!(download -> architecture (architecture_id));
joinable!(download -> build (build_id));
joinable!(icon -> version (version_id));
joinable!(package -> user (author_user_id));
joinable!(package_user_maintainer -> package (package_id));
joinable!(package_user_maintainer -> user (user_id));
joinable!(screenshot -> package (package_id));
joinable!(user_role -> role (role_id));
joinable!(user_role -> user (user_id));
joinable!(version -> package (package_id));
joinable!(version_service_dependency -> package (package_id));
joinable!(version_service_dependency -> version (version_id));

allow_tables_to_appear_in_same_query!(
    architecture,
    build,
    build_architecture,
    description,
    displayname,
    download,
    firmware,
    icon,
    language,
    package,
    package_user_maintainer,
    role,
    screenshot,
    service,
    user,
    user_role,
    version,
    version_service_dependency,
);
