use crate::models::Wrapper;
use crate::Database;
use crate::HashMap;
use axum::body::StreamBody;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use regex::Regex;
use serde_json::Value;
use tokio_util::io::ReaderStream;

macro_rules! wrapping {
    ($result: expr) => {
        $crate::util::wrap_result($result, None)
    };

    ($result: expr, $wrap: expr) => {
        $crate::util::wrap_result($result, Some($wrap))
    };
}

pub async fn query_data(
    Path(path_map): Path<HashMap<String, String>>,
    Query(query): Query<HashMap<String, String>>,
    Extension(db): Extension<Database>,
) -> impl IntoResponse {
    wrapping!(db.query_data(&path_map, &query))
}

pub async fn get_data(
    Path(path_map): Path<HashMap<String, String>>,
    Extension(db): Extension<Database>,
    Extension(wrap): Extension<Wrapper>,
) -> impl IntoResponse {
    wrapping!(db.get_data(&path_map), wrap)
}

pub async fn post_data(
    Path(path_map): Path<HashMap<String, String>>,
    Json(body): Json<Value>,
    Extension(mut db): Extension<Database>,
    Extension(wrap): Extension<Wrapper>,
) -> impl IntoResponse {
    wrapping!(db.create_data(&path_map, body), wrap)
}

pub async fn put_data(
    Path(query): Path<HashMap<String, String>>,
    Json(body): Json<Value>,
    Extension(mut db): Extension<Database>,
    Extension(wrap): Extension<Wrapper>,
) -> impl IntoResponse {
    wrapping!(db.update_data(&query, body), wrap)
}

pub async fn delete_data(
    Path(query): Path<HashMap<String, String>>,
    Extension(mut db): Extension<Database>,
    Extension(wrap): Extension<Wrapper>,
) -> impl IntoResponse {
    wrapping!(db.delete_data(&query), wrap)
}

pub async fn get_file(
    Path(query): Path<HashMap<String, String>>,
    Extension(mut db): Extension<Database>,
) -> impl IntoResponse {
    match db.get_file(&query) {
        Ok(ref path) => {
            let file = match tokio::fs::File::open(path).await {
                Ok(file) => file,
                Err(err) => {
                    return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err)))
                }
            };
            let stream = ReaderStream::new(file);
            let body = StreamBody::new(stream);

            let mut content_type = "application/octet-stream".to_string();
            let file_path = std::path::Path::new(path);
            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                if Regex::new("jpg|png|gif|jpeg").unwrap().is_match(ext) {
                    content_type = format!("image/{}; charset=utf-8", ext);
                }

                if Regex::new("json").unwrap().is_match(ext) {
                    content_type = format!("application/{}; charset=utf-8", ext);
                }

                if Regex::new("txt").unwrap().is_match(ext) {
                    content_type = format!("text/plain; charset=utf-8");
                }
            }

            let headers = [(header::CONTENT_TYPE, content_type)];
            Ok((headers, body))
        }

        Err(err) => return Err((StatusCode::NOT_FOUND, err)),
    }
}
