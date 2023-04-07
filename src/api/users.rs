use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use crate::models::app_state::AppState;
use crate::models::schemas::{CreateUserSchema, ErrorResponse};
use crate::models::users::User;

#[get("/users")]
pub async fn users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&state.pool)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e.to_string() }),
    }
}

#[get("/users/{id}")]
pub async fn find_user(state: Data<AppState>, user_id: Path<String>) -> impl Responder {
    if uuid::Uuid::parse_str(&user_id).is_err() {
        return HttpResponse::BadRequest().json(ErrorResponse { error: "Invalid user id".to_string() });
    }

    match sqlx::query_as::<_, User>(
        "SELECT id, name FROM users WHERE id = $1::uuid",
    )
        .bind(user_id.into_inner())
        .fetch_one(&state.pool)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e.to_string() }),
    }
}

#[post("/users")]
pub async fn create_user(
    state: Data<AppState>,
    schema: Json<CreateUserSchema>,
) -> impl Responder {
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name) VALUES ($1, $2) RETURNING id, name",
    )
        .bind(&uuid::Uuid::new_v4())
        .bind(&schema.name)
        .fetch_one(&state.pool)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e.to_string() }),
    }
}
