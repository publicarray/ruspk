table! {
    architecture (id) {
        id -> Unsigned<Bigint>,
        code -> Varchar,
    }
}

table! {
    build (id) {
        id -> Unsigned<Bigint>,
        version_id -> Unsigned<Bigint>,
        firmware_id -> Unsigned<Bigint>,
        publisher_user_id -> Nullable<Unsigned<Bigint>>,
        checksum -> Nullable<Varchar>,
        extract_size -> Integer,
        path -> Varchar,
        md5 -> Varchar,
        insert_date -> Datetime,
        active -> Nullable<Bool>,
    }
}

table! {
    build_architecture (build_id, architecture_id) {
        build_id -> Unsigned<Bigint>,
        architecture_id -> Unsigned<Bigint>,
    }
}

table! {
    description (version_id, language_id) {
        version_id -> Unsigned<Bigint>,
        language_id -> Unsigned<Bigint>,
        #[sql_name = "description"]
        desc -> Text,
    }
}

table! {
    displayname (version_id, language_id) {
        version_id -> Unsigned<Bigint>,
        language_id -> Unsigned<Bigint>,
        #[sql_name = "displayname"]
        name -> Varchar,
    }
}

table! {
    download (id) {
        id -> Unsigned<Bigint>,
        build_id -> Unsigned<Bigint>,
        architecture_id -> Unsigned<Bigint>,
        firmware_build -> Integer,
        ip_address -> Varchar,
        user_agent -> Nullable<Varchar>,
        date -> Datetime,
    }
}

table! {
    firmware (id) {
        id -> Unsigned<Bigint>,
        version -> Varchar,
        build -> Unsigned<Bigint>,
    }
}

table! {
    icon (id) {
        id -> Unsigned<Bigint>,
        version_id -> Unsigned<Bigint>,
        size -> Integer,
        path -> Varchar,
    }
}

table! {
    language (id) {
        id -> Unsigned<Bigint>,
        code -> Varchar,
        name -> Varchar,
    }
}

table! {
    package (id) {
        id -> Unsigned<Bigint>,
        author_user_id -> Nullable<Unsigned<Bigint>>,
        name -> Varchar,
        insert_date -> Nullable<Datetime>,
    }
}

table! {
    package_user_maintainer (package_id, user_id) {
        package_id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
    }
}

table! {
    role (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    screenshot (id) {
        id -> Unsigned<Bigint>,
        package_id -> Unsigned<Bigint>,
        path -> Varchar,
    }
}

table! {
    service (id) {
        id -> Unsigned<Bigint>,
        code -> Varchar,
    }
}

table! {
    user (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        api_key -> Nullable<Varchar>,
        github_access_token -> Nullable<Varchar>,
        active -> Bool,
        confirmed_at -> Nullable<Datetime>,
    }
}

table! {
    user_role (user_id, role_id) {
        user_id -> Unsigned<Bigint>,
        role_id -> Unsigned<Bigint>,
    }
}

table! {
    version (id) {
        id -> Unsigned<Bigint>,
        package_id -> Unsigned<Bigint>,
        #[sql_name = "version"]
        ver -> Unsigned<Integer>,
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
        insert_date -> Datetime,
    }
}

table! {
    version_service_dependency (version_id, package_id) {
        version_id -> Unsigned<Bigint>,
        package_id -> Unsigned<Bigint>,
    }
}

joinable!(build -> firmware (firmware_id));
joinable!(build -> version (version_id));
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
