use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::{PgPool, query};

#[derive(Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if form.email.is_empty() || form.name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    let _result = query!(
        r#"
        INSERT INTO subscriptions (email, name)
        VALUES ($1, $2)
        "#,
        form.email,
        form.name
    );

    dbg!(&form);

    HttpResponse::Ok().finish()
}
