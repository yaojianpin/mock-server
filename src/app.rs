use crate::{service::*, Database, HashMap};
use axum::{
    extract::{OriginalUri, Path, Query},
    http::{Method, StatusCode},
    response::{IntoResponse},
    routing::{any, get, get_service},
    Extension, Json, Router,
};
use serde_json::Value;
use tower_http::services::ServeDir;

const DATA_QUERY_TPL: &str = "/api/([^/]*)$";
const DATA_ID_TPL: &str = "/api/([^/]*)/([^/]*)$";

pub fn create(_: &Database) -> Router {
    let app = Router::new()
        .route("/api/:data", get(query_data).post(query_data))
        .route(
            "/api/:data/:id",
            get(get_data)
                .post(post_data)
                .put(put_data)
                .delete(delete_data),
        )
        .fallback(get_service(ServeDir::new("static")).handle_error(handle_error));
    //.layer(TraceLayer::new_for_http())

    app
}

pub fn proxy(db: &Database) -> Router {
    let config = db.get_config();
    let mut router = Router::new();
    for (key, v) in &config.routing {
        tracing::debug!("routing {} to {:?}", key, v);
        let routing_value = v.clone();
        router = router.route(
            key,
            any(
                |method: Method,
                 mut path: Path<HashMap<String, String>>,
                 query: Query<HashMap<String, String>>,
                 body: Json<Value>,
                 db: Extension<Database>,
                 OriginalUri(original_uri): OriginalUri| async move {
                    tracing::debug!(
                        "original_uri = {original_uri}, method={method} path={:?}, query={:?}, body={:?}",
                        path, query, body
                    );
                   
                    // match the template
                    let re = regex::Regex::new(DATA_QUERY_TPL).unwrap();
                    if let Some(cap) = re.captures(&routing_value.to) {
                        let data = cap.get(1).unwrap().as_str();
                        path.insert("data".to_string(), data.to_string());

                        let mut new_query = create_query(query, routing_value.query);
                        return query_data(path, Query(new_query), db.clone())
                            .await
                            .into_response();
                    }

                    let re = regex::Regex::new(DATA_ID_TPL).unwrap();
                   if let Some(cap) = re.captures(&routing_value.to) {
                        let data = cap.get(1).unwrap().as_str();
                        let id = cap.get(2).unwrap().as_str();
                        path.insert("data".to_string(), data.to_string());
                        path.insert("id".to_string(), id.to_string());

                        return match method {
                            Method::GET => get_data(path, db.clone()).await.into_response(),
                            Method::POST => post_data(path, body, db).await.into_response(),
                            Method::PUT => put_data(path, body, db).await.into_response(),
                            Method::DELETE => delete_data(path, db).await.into_response(),
                            _ => (StatusCode::METHOD_NOT_ALLOWED, "method not support").into_response(),
                        }
                    }

                    (StatusCode::BAD_REQUEST, "bad request").into_response()
                },
            ),
        )
    }

    router
}

async fn handle_error(_: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "server error")
}

fn create_query(Query(query): Query<HashMap<String, String>>, map: Option<HashMap<String, String>>) -> HashMap<String, String> {
    let mut new_query = HashMap::new();
    match map {
        Some(q) => {
            for key in query.keys() {
                let value = &query[key];
                if let Some(map_key) = q.get(key) {
                    if map_key != "_" {
                        new_query.insert(map_key.to_string(), value.to_string());
                    }
                }
            }
        },
        None=> {
            new_query = query.clone();
        }
    }
    new_query
}
