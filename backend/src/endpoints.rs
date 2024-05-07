use crate::model::{GetUrls, PostUrls};
use crate::{config::AppState, Actions};
use axum::{extract::State, http::StatusCode, Json};

pub async fn return_url(
    State(app_state): State<AppState>,
    Json(payload): Json<GetUrls>,
) -> (StatusCode, Json<PostUrls>) {
    let status = StatusCode::OK;

    let to_url = crate::run(
        Actions::GetRedirectUrl(payload.from_url.clone()),
        app_state.config,
        app_state.pool,
    )
    .await;

    let from_url = payload.from_url;
    let urls = PostUrls {
        from_url,
        // TODO: more graceful error handling
        to_url: to_url.expect("URL Nenalezeno!"),
    };
    let response: Json<PostUrls> = Json(urls);
    (status, response)
}

pub async fn set_url(
    State(app_state): State<AppState>,
    Json(payload): Json<GetUrls>,
) -> (StatusCode, Json<PostUrls>) {
    unimplemented!();
}
