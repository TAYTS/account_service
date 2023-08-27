pub mod route;

#[tokio::main]
async fn main() {
    let router = route::get_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
