use crate::Database;
use crate::HashMap;
use axum::extract::Path;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde_json::Value;

macro_rules! wrapping {
  ($result: expr) => {
    $crate::db::get_config().wrapping_result($result)
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
) -> impl IntoResponse {
  wrapping!(db.get_data(&path_map))
}

pub async fn post_data(
  Path(path_map): Path<HashMap<String, String>>,
  Json(body): Json<Value>,
  Extension(mut db): Extension<Database>,
) -> impl IntoResponse {
  wrapping!(db.create_data(&path_map, body))
}

pub async fn put_data(
  Path(query): Path<HashMap<String, String>>,
  Json(body): Json<Value>,
  Extension(mut db): Extension<Database>,
) -> impl IntoResponse {
  wrapping!(db.update_data(&query, body))
}

pub async fn delete_data(
  Path(query): Path<HashMap<String, String>>,
  Extension(mut db): Extension<Database>,
) -> impl IntoResponse {
  wrapping!(db.delete_data(&query))
}
