// @generated automatically by Diesel CLI.

diesel::table! {
    package_authors (package, user_id) {
        package -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    package_relations (package, dependency, kind) {
        package -> Int4,
        dependency -> Int4,
        kind -> Int4,
    }
}

diesel::table! {
    package_version_refs (value) {
        value -> Int4,
    }
}

diesel::table! {
    package_versions (id) {
        id -> Int4,
        package -> Int4,
        name -> Text,
        version_number -> Text,
        file_id -> Text,
        changelog -> Nullable<Text>,
        loaders -> Array<Nullable<Text>>,
        game_versions -> Array<Nullable<Text>>,
        downloads -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    packages (id) {
        id -> Int4,
        name -> Text,
        slug -> Text,
        readme -> Text,
        description -> Text,
        views -> Int4,
        source -> Nullable<Text>,
        issues -> Nullable<Text>,
        wiki -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        downloads -> Int4,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        value -> Text,
        expires -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        github_id -> Int4,
    }
}

diesel::joinable!(package_authors -> packages (package));
diesel::joinable!(package_authors -> users (user_id));
diesel::joinable!(package_relations -> package_version_refs (dependency));
diesel::joinable!(package_relations -> package_versions (package));
diesel::joinable!(package_version_refs -> package_versions (value));
diesel::joinable!(package_versions -> packages (package));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    package_authors,
    package_relations,
    package_version_refs,
    package_versions,
    packages,
    user_tokens,
    users,
);
