use axum::{http::StatusCode, response::Json, routing::get, Router};

pub struct UsersService {}

use crate::route::ServiceRoute;

impl ServiceRoute for UsersService {
    fn register_route(router: Router) -> Router {
        router.route("/users", get(list_users))
    }
}

async fn list_users() -> Result<Json<Vec<String>>, StatusCode> {
    return Ok(Json(vec![String::from("John"), String::from("Helen")]));
}
