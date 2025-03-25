use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Tmpl {}

pub async fn root() -> impl IntoResponse {
    let tmpl = Tmpl {};
    Html(tmpl.render().unwrap()).into_response()
}
