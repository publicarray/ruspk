table! {
    architecture (id) {
        id -> BigInt,
        code -> Text,
    }
}

table! {
    build (id) {
        id -> BigInt,
        version_id -> BigInt,
        firmware_id -> BigInt,
        publisher_user_id -> Nullable<BigInt>,
        checksum -> Nullable<Text>,
        extract_size -> Nullable<Integer>,
        path -> Text,
        md5 -> Nullable<Text>,
        insert_date -> Timestamp,
        active -> Nullable<Bool>,
    }
}

table! {
    build_architecture (build_id, architecture_id) {
        build_id -> Nullable<BigInt>,
        architecture_id -> Nullable<BigInt>,
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
        name -> Text,
    }
}

table! {
    download (id) {
        id -> BigInt,
        build_id -> BigInt,
        architecture_id -> BigInt,
        firmware_build -> Integer,
        ip_address -> Text,
        user_agent -> Nullable<Text>,
        date -> Timestamp,
    }
}

table! {
    firmware (id) {
        id -> BigInt,
        version -> Text,
        build -> Integer,
    }
}

table! {
    icon (id) {
        id -> BigInt,
        version_id -> BigInt,
        size -> Integer,
        path -> Text,
    }
}

table! {
    language (id) {
        id -> BigInt,
        code -> Text,
        name -> Text,
    }
}

table! {
    package (id) {
        id -> BigInt,
        author_user_id -> Nullable<BigInt>,
        name -> Text,
        insert_date -> Nullable<Timestamp>,
    }
}

table! {
    package_user_maintainer (package_id, user_id) {
        package_id -> Nullable<BigInt>,
        user_id -> Nullable<BigInt>,
    }
}

table! {
    role (id) {
        id -> BigInt,
        name -> Text,
        description -> Text,
    }
}

table! {
    screenshot (id) {
        id -> BigInt,
        package_id -> BigInt,
        path -> Text,
    }
}

table! {
    service (id) {
        id -> BigInt,
        code -> Text,
    }
}

table! {
    user (id) {
        id -> BigInt,
        username -> Text,
        email -> Text,
        password -> Text,
        api_key -> Nullable<Text>,
        github_access_token -> Nullable<Text>,
        active -> Bool,
        confirmed_at -> Nullable<Timestamp>,
    }
}

table! {
    user_role (user_id, role_id) {
        user_id -> Nullable<BigInt>,
        role_id -> Nullable<BigInt>,
    }
}

table! {
    version (id) {
        id -> BigInt,
        package_id -> BigInt,
        #[sql_name = "version"]
        ver -> Integer,
        upstream_version -> Text,
        changelog -> Nullable<Text>,
        report_url -> Nullable<Text>,
        distributor -> Nullable<Text>,
        distributor_url -> Nullable<Text>,
        maintainer -> Nullable<Text>,
        maintainer_url -> Nullable<Text>,
        dependencies -> Nullable<Text>,
        conf_dependencies -> Nullable<Text>,
        conflicts -> Nullable<Text>,
        conf_conflicts -> Nullable<Text>,
        install_wizard -> Nullable<Bool>,
        upgrade_wizard -> Nullable<Bool>,
        startable -> Nullable<Bool>,
        license -> Nullable<Text>,
        insert_date -> Timestamp,
    }
}

table! {
    version_service_dependency (version_id, package_id) {
        version_id -> Nullable<BigInt>,
        package_id -> Nullable<BigInt>,
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
