use crate::models::{AddTokenPayload, Token};
use crate::routes::*;

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(
    paths(add_token,get_token,get_all_tokens,get_all_emails),
    components(schemas(AddTokenPayload, Token)),
    tags((name = "api", description = "API documentation"))
)]
pub struct ApiDoc;
