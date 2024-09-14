use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

#[derive(Debug, Clone)]
struct ServerState {
    pool: sqlx::PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = ServerState { pool };
    let router = Router::new()
        .route(
            "/",
            get(|| async { Redirect::to("https://github.com/FineFindus/Hieroglyphic") }),
        )
        .route("/v1/upload/:label", post(upload_data))
        .with_state(state);

    Ok(router.into())
}

async fn upload_data(
    Path(label): Path<String>,
    State(state): State<ServerState>,
    Json(strokes): Json<Vec<detexify::Stroke>>,
) -> StatusCode {

    // validate and prepare strokes
    let Some(sample) = detexify::StrokeSample::new(strokes) else {
        tracing::error!("Invalid strokes");
        return StatusCode::BAD_REQUEST;
    };

    // add label into db, if not exits
    let Ok((id, label)): Result<(i32, String), sqlx::Error> = sqlx::query_as(
        "INSERT INTO labels (name) VALUES ($1)
     ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
     RETURNING id, name",
    )
    .bind(&label)
    .fetch_one(&state.pool)
    .await
    else {
        tracing::error!("Failed to insert label '{}' into db", &label);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    // add strokes into db with relation to label
    let Ok(json) = serde_json::to_value(sample) else {
        tracing::error!("Failed to convert data to json");
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    if let Err(err) = sqlx::query("INSERT INTO samples (label_id, strokes) VALUES ($1, $2)")
        .bind(id)
        .bind(&json)
        .execute(&state.pool)
        .await
    {
        tracing::error!("Failed to insert strokes for '{}' into db: {}", &label, err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
