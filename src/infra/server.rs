use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use tokio_util::io::ReaderStream;

use crate::{
    di::AppState,
    infra::{
        db::{DownloadLinksMapperImpl, YtHistoryMapperImpl},
        dto::RegisterResponse,
    },
};

type App = AppState<YtHistoryMapperImpl, DownloadLinksMapperImpl>;

pub async fn get_route(state: App) -> Router {
    let rfserve = Router::new()
        .route("/download/{fileid}", get(download_file))
        .route("/create/{id}", post(create_download_links))
        .with_state(state);

    Router::new().nest("/rfserve", rfserve)
}

async fn download_file(Path(fileid): Path<String>, State(app): State<App>) -> impl IntoResponse {
    let service = app.download_service;
    let output = match service.execute(fileid.clone()).await {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("download処理にてerror発生 file_id:{} ERROR:{}", &fileid, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "app error").into_response();
        }
    };
    let stream = ReaderStream::new(output.file);
    let body = axum::body::Body::from_stream(stream);
    (StatusCode::OK, create_download_header(&output.fname), body).into_response()
}

async fn create_download_links(
    Path(id): Path<i64>,
    State(app): State<App>,
) -> (StatusCode, Json<RegisterResponse>) {
    let result = match app.register_service.execute(id).await {
        Ok(o) => o,
        Err(e) => {
            tracing::error!("登録処理にてエラー発生 yt_history_id:{} ERROR:{}", id, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterResponse::new(None, Some("app error".into()))),
            );
        }
    };
    (
        StatusCode::OK,
        Json(RegisterResponse::new(Some(result.url), None)),
    )
}

fn create_download_header(fname: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", fname)
            .parse()
            .unwrap(),
    );

    headers
}
