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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--> Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
