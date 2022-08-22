use crate::service::*;
use crate::Database;
use crate::HashMap;
use axum::extract::OriginalUri;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::any;
use axum::routing::get;
use axum::routing::get_service;
use axum::Extension;
use axum::Json;
use axum::Router;
use tower_http::services::ServeDir;

pub fn create(_: &Database) -> Router {
  let app = Router::new()
    .nest(
      "/api",
      Router::new()
        .route("/:data", get(query_data).post(query_data))
        .route(
          "/:data/:id",
          get(get_data)
            .post(post_data)
            .put(put_data)
            .delete(delete_data),
        ),
    )
    .fallback(get_service(ServeDir::new("static")).handle_error(handle_error));
  //.layer(TraceLayer::new_for_http())

  app
}

pub fn proxy(db: &Database) -> Router {
  let config = db.get_config();
  let mut router = Router::new();
  for (rk, rv) in &config.routing {
    tracing::debug!("routing {} to {:?}", rk, rv);
    let key = rk.clone();
    let value = rv.clone();
    router = router.route(
      &rk,
      any(
        |Path(path_map): Path<HashMap<String, String>>,
         Query(query_map): Query<HashMap<String, String>>,
         OriginalUri(original_uri): OriginalUri| async move {
          tracing::debug!("original_uri = {original_uri}, path_map={:?}", path_map);
          let mut from = key;
          let mut to = value.to;
          for (pk, pv) in path_map {
            to = to.replace(&format!(":{pk}"), &pv);
            from = from.replace(&format!(":{pk}"), &pv);
          }
          let mut query = (original_uri.clone().to_string()).replace(&from, "");

          if let Some(q) = value.query {
            for (qk, _) in query_map {
              if let Some(k) = q.get(&qk) {
                if k != "_" {
                  query = query.replace(&qk, &k);
                }
              }
            }
          }

          let to = to + &query;
          tracing::debug!("redirect to {to}");
          Redirect::temporary(&to)
        },
      ),
    )
  }

  router
}

async fn handle_error(_: std::io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "server error")
}
