use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "ModHost API",
        description = "The ModHost REST API.",

        license(
            name = "MIT",
            url = "https://opensource.org/license/mit/",
        ),
    ),
    paths(
        crate::routes::api::yaml_api,
        crate::routes::api::json_api,
        crate::routes::users::me::me_handler,
        crate::routes::users::info::info_handler,
        crate::routes::users::pkg::list_handler,
        crate::routes::users::search::search_handler,
        crate::routes::auth::login::login_handler,
        crate::routes::auth::callback::callback_handler,
        crate::routes::pkg::info::info_handler,
        crate::routes::pkg::info::update_handler,
        crate::routes::pkg::info::delete_handler,
        crate::routes::pkg::list::list_handler,
        crate::routes::pkg::list::create_handler,
        crate::routes::pkg::ver::list_handler,
        crate::routes::pkg::ver::info_handler,
        crate::routes::pkg::ver::download_handler,
        crate::routes::pkg::ver::create_handler,
        crate::routes::pkg::ver::update_handler,
        crate::routes::pkg::ver::delete_handler,
        crate::routes::pkg::ver::latest_handler,
        crate::routes::pkg::author::list_handler,
        crate::routes::pkg::author::add_handler,
        crate::routes::pkg::author::remove_handler,
        crate::routes::pkg::search::search_handler,
        crate::routes::meta::badge::version_handler,
        crate::routes::meta::vers::game_versions_handler,
        crate::routes::meta::loaders::loaders_handler,
    ),
    components(
        schemas(
            db::User,
            db::UserToken,
            db::NewUser,
            db::NewUserToken,
            db::PackageManifest,
            db::Package,
            db::PackageAuthor,
            db::PackageRelation,
            db::PackageVersion,
            db::PackageVersionRef,
            db::PackageVersionInit,
            db::NewPackage,
            db::NewPackageVersion,
            db::RelationKind,
            db::PackageData,
            crate::routes::api::JsonQueryParams,
            crate::routes::users::search::SearchQuery,
            crate::routes::pkg::info::PartialPackage,
            crate::routes::pkg::ver::PartialPackageVersion,
            crate::routes::pkg::search::SearchQuery,
            crate::routes::meta::vers::GameVersion,
            crate::routes::meta::loaders::ModLoader,
        ),
        responses(
            db::User,
            db::UserToken,
            db::NewUser,
            db::NewUserToken,
            db::PackageManifest,
            db::Package,
            db::PackageAuthor,
            db::PackageRelation,
            db::PackageVersion,
            db::PackageVersionRef,
            db::PackageVersionInit,
            db::NewPackage,
            db::NewPackageVersion,
            db::RelationKind,
            db::PackageData,
            crate::routes::api::JsonQueryParams,
            crate::routes::users::search::SearchQuery,
            crate::routes::pkg::info::PartialPackage,
            crate::routes::pkg::ver::PartialPackageVersion,
            crate::routes::pkg::search::SearchQuery,
            crate::routes::meta::vers::GameVersion,
            crate::routes::meta::loaders::ModLoader,
        ),
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints."),
        (name = "Users", description = "User-related endpoints."),
        (name = "Packages", description = "Package-related endpoints."),
        (name = "Versions", description = "Package version-related endpoints."),
        (name = "Misc", description = "Miscellaneous endpoints."),
        (name = "Meta", description = "Metadata-related endpoints."),
    ),
    modifiers(
        &TokenAuthAddon,
    ),
)]
pub struct ApiDocs;

#[derive(Debug, Clone, Copy)]
pub struct TokenAuthAddon;

impl Modify for TokenAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "api_auth_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("TOKEN")
                    .build(),
            ),
        )
    }
}
