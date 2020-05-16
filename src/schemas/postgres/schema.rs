table! {
    architecture (id) {
        id -> BigInt,
        code -> Varchar,
    }
}

table! {
    build (id) {
        id -> BigInt,
        package_id -> BigInt,
        firmware_id -> BigInt,
        publisher_user_id -> Nullable<BigInt>,
        checksum -> Nullable<Varchar>,
        extract_size -> Int4,
        path -> Varchar,
        md5 -> Varchar,
        insert_date -> Timestamp,
        active -> Nullable<Bool>,
    }
}

table! {
    build_architecture (build_id, architecture_id) {
        build_id -> BigInt,
        architecture_id -> BigInt,
    }
}

table! {
    description (version_id, language_id) {
        version_id -> BigInt,
        language_id -> BigInt,
        #[sql_name = "description"]
        desc -> Text,
    }
}

table! {
    displayname (version_id, language_id) {
        version_id -> BigInt,
        language_id -> BigInt,
        #[sql_name = "displayname"]
        name -> Varchar,
    }
}

table! {
    download (id) {
        id -> BigInt,
        build_id -> BigInt,
        architecture_id -> BigInt,
        firmware_build -> BigInt,
        ip_address -> Varchar,
        user_agent -> Nullable<Varchar>,
        date -> Timestamp,
    }
}

table! {
    firmware (id) {
        id -> BigInt,
        version -> Varchar,
        build -> BigInt,
    }
}

table! {
    icon (id) {
        id -> BigInt,
        version_id -> BigInt,
        size -> Int4,
        path -> Varchar,
    }
}

table! {
    language (id) {
        id -> BigInt,
        code -> Varchar,
        name -> Varchar,
    }
}

table! {
    package (id) {
        id -> BigInt,
        author_user_id -> Nullable<BigInt>,
        name -> Varchar,
        insert_date -> Nullable<Timestamp>,
    }
}

table! {
    package_user_maintainer (package_id, user_id) {
        package_id -> BigInt,
        user_id -> BigInt,
    }
}

table! {
    role (id) {
        id -> BigInt,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    screenshot (id) {
        id -> BigInt,
        package_id -> BigInt,
        path -> Varchar,
    }
}

table! {
    service (id) {
        id -> BigInt,
        code -> Varchar,
    }
}

table! {
    user (id) {
        id -> BigInt,
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
        user_id -> BigInt,
        role_id -> BigInt,
    }
}

table! {
    version (id) {
        id -> BigInt,
        package_id -> BigInt,
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
        version_id -> BigInt,
        package_id -> BigInt,
    }
}

joinable!(build -> firmware (firmware_id));
joinable!(build -> package (package_id));
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
joinable!(user_role -> role (user_id));
joinable!(user_role -> user (role_id));
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
