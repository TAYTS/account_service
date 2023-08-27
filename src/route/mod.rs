use axum::Router;
mod users;

use crate::route::users::UsersService;

pub fn get_router() -> Router {
    let router = Router::new();
    UsersService::register_route(router)
}

trait ServiceRoute {
    fn register_route(router: Router) -> Router;
}
