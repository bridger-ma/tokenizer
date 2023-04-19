use std::sync::Arc;

use axum::Router;
use grust_light::{controllers::TokenController, errors::Error, openapi::ApiDoc, routes::routes};
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> ! {
    // Initialize the redis client
    let redis_uri: String =
        "redis://default:hT1h9eod7UqWrvGFX4IiBWNKkRSbXJed@redis-14312.c17.us-east-1-4.ec2.cloud.redislabs.com:14312"
            .into();
    let client = redis::Client::open(redis_uri.clone())
        .map_err(|e| Error::FailToParseRedisUri {
            uri: redis_uri.clone(),
            message: e.to_string(),
        })
        .unwrap();
    let conn = client
        .get_async_connection()
        .await
        .map_err(|e| Error::FailToParseRedisUri {
            uri: redis_uri,
            message: e.to_string(),
        })
        .unwrap();
    let conn = Arc::new(Mutex::new(conn));
    // Initialize the logger

    // Initialize the models and controllers
    let token_controller = TokenController::new(Arc::clone(&conn));
    let tc_clone = token_controller.clone();
    //initialize the routes
    let app_routes = routes(token_controller);
    // spawn token refresh task

    tokio::spawn(async move {
        let router = Router::new()
            .nest("/api", app_routes)
            .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()));
        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    loop {
        let token = tc_clone.refresh_all_tokens().await;
        match token {
            Ok(token) => {
                println!("token refreshed: {:?}", token);
            }
            Err(e) => {
                println!("token refresh failed: {:?}", e);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(20 * 60)).await;
    }
    // Create the router
}
