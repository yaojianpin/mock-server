use crate::{
    models::{DataConfig, RoutingRule, Wrapper, WRAP_KEY_ERR, WRAP_KEY_OK},
    service::*,
    util, Database, HashMap,
};
use axum::{
    extract::{OriginalUri, Path, Query},
    http::{Method, StatusCode},
    response::IntoResponse,
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

    app
}

pub fn proxy(db: &Database) -> Router {
    let config = db.get_config();
    let mut router = Router::new();
    for (key, v) in &config.routing {
        tracing::debug!("routing {} to {:?}", key, v);
        let routing_value = v.clone();
        let wrap = create_wrap(config, &routing_value.wrapping);
        router = router.route(
            key,
            any(
                |method: Method,
                 mut path: Path<HashMap<String, String>>,
                 query: Query<HashMap<String, String>>,
                 body: Json<Value>,
                 db: Extension<Database>,
                 wapper: Extension<Wrapper>,
                 OriginalUri(original_uri): OriginalUri| async move {
                    tracing::debug!(
                        "original_uri = {original_uri}, method={method} path={:?}, query={:?}, body={:?}",
                        path, query, body
                    );

                    // 检查规则
                    if let Err(err) = validate_rules(routing_value.rules, &path, &query, &body) {
                        return util::wrap_result(Err(err), Some(wapper.0)).into_response();
                    }
                    // match the template
                    let re = regex::Regex::new(DATA_QUERY_TPL).unwrap();
                    if let Some(cap) = re.captures(&routing_value.to) {
                        let data = cap.get(1).unwrap().as_str();
                        if !data.starts_with(":") {
                            path.insert("data".to_string(), data.to_string());
                        }

                        let new_query = create_query(query, routing_value.query);
                        return query_data(path, Query(new_query), db.clone())
                            .await
                            .into_response();
                    }

                    let re = regex::Regex::new(DATA_ID_TPL).unwrap();
                   if let Some(cap) = re.captures(&routing_value.to) {
                        let data = cap.get(1).unwrap().as_str();
                        let id = cap.get(2).unwrap().as_str();
                        if !data.starts_with(":") {
                            path.insert("data".to_string(), data.to_string());
                        }

                        if !id.starts_with(":") {
                            path.insert("id".to_string(), id.to_string());
                        }
                        return match method {
                            Method::GET => get_data(path, db, wapper).await.into_response(),
                            Method::POST => post_data(path, body, db, wapper).await.into_response(),
                            Method::PUT => put_data(path, body, db, wapper).await.into_response(),
                            Method::DELETE => delete_data(path, db, wapper).await.into_response(),
                            _ => (StatusCode::METHOD_NOT_ALLOWED, "method not support").into_response(),
                        }
                    }

                    (StatusCode::BAD_REQUEST, "bad request").into_response()
                },
            ),
        ).layer(Extension(wrap))
    }

    router
}

async fn handle_error(_: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "server error")
}

fn create_query(
    Query(query): Query<HashMap<String, String>>,
    map: Option<HashMap<String, String>>,
) -> HashMap<String, String> {
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
        }
        None => {
            new_query = query.clone();
        }
    }
    new_query
}

fn validate_rules(
    rules: Option<Vec<RoutingRule>>,
    Path(path): &Path<HashMap<String, String>>,
    Query(query): &Query<HashMap<String, String>>,
    Json(body): &Json<Value>,
) -> Result<bool, String> {
    if let Some(rules) = rules {
        for rule in rules {
            // match the path value
            if path.contains_key(&rule.key) {
                let re = regex::Regex::new(&rule.r#match).unwrap();
                if !re.is_match(&path.get(&rule.key).unwrap()) {
                    return Err(rule.message);
                }
            }

            // match the query value
            if query.contains_key(&rule.key) {
                let re = regex::Regex::new(&rule.r#match).unwrap();
                if !re.is_match(&query.get(&rule.key).unwrap()) {
                    return Err(rule.message);
                }
            }

            // match the body value
            match &body {
                Value::Array(arr) => {
                    for v in arr {
                        match v {
                            Value::Object(obj) => {
                                if obj.contains_key(&rule.key) {
                                    let re = regex::Regex::new(&rule.r#match).unwrap();
                                    if !re.is_match(&obj.get(&rule.key).unwrap().to_string()) {
                                        return Err(rule.message);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Value::Object(obj) => {
                    if obj.contains_key(&rule.key) {
                        let re = regex::Regex::new(&rule.r#match).unwrap();
                        if !re.is_match(&obj.get(&rule.key).unwrap().to_string()) {
                            return Err(rule.message);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(true)
}

fn create_wrap(config: &DataConfig, routing_wrapper: &Option<Wrapper>) -> Wrapper {
    let mut wrapper = config.wrapping.clone();
    if let Some(routing_wrapper) = routing_wrapper {
        if let Some(ok) = routing_wrapper.get(WRAP_KEY_OK) {
            wrapper
                .entry(WRAP_KEY_OK.to_string())
                .and_modify(|entry| *entry = ok.clone())
                .or_insert(ok.clone());
        }

        if let Some(err) = routing_wrapper.get(WRAP_KEY_ERR) {
            wrapper
                .entry(WRAP_KEY_ERR.to_string())
                .and_modify(|entry| *entry = err.clone())
                .or_insert(err.clone());
        }
    }
    wrapper.clone()
}
