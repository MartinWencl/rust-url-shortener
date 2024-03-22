use crate::{config::AppState, Actions};
use crate::model::{PostUrls, GetUrls};
use axum::{
    extract::State, http::StatusCode, Json
};


pub async fn return_url(State(app_state): State<AppState>, Json(payload): Json<GetUrls>) -> (StatusCode, Json<PostUrls>) {
    let status = StatusCode::OK;

    let from_url = crate::run(Actions::GetRedirectUrl(payload.from_url.clone()), app_state.config, app_state.pool).await;
    let to_url = payload.from_url;
    let urls = PostUrls {
        from_url: from_url.expect("aaa"),
        to_url,
    };
    let response: Json<PostUrls> = Json(urls);
    log::debug!("{:?}\t{:?}\t{:?}", status, response.from_url, response.to_url);
    (status, response)
}

pub async fn set_url(State(app_state): State<AppState>, Json(payload): Json<GetUrls>) -> (StatusCode, Json<PostUrls>) {
    unimplemented!();
}
