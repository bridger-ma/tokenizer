use axum::Router;
use grust_light::{controllers::TokenController, errors::Result, openapi::ApiDoc, routes::routes};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger

    // Initialize the models and controllers
    let token_controller = TokenController::new();
    //initialize the routes
    let app_routes = routes(token_controller);
    // Create the router
    let router = Router::new()
        .nest("/api", app_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
