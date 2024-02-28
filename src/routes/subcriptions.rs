use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::{PgPool, query};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(_pool.get_ref())
    .await
    .expect("Failed to Execute Query");
    HttpResponse::Ok().finish()
}
