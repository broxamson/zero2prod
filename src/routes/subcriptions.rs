use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::{PgPool, query};
use uuid::Uuid;
use tracing::{Instrument};


#[derive(Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_nam = %form.name
    );
    let _request_span_gaurd = request_span.enter();
    
    tracing::info!("Saving new Subscriber details in the database");
    tracing::info!("request_id {} -- Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    let query_span = tracing::info_span!("Saving new subscriber details to the databae");
    match query!(
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
        .instrument(query_span)
    .await
    {
        Ok(_)=> {
            tracing::info!(
        "request_id {} -- New Subscriber details have been saved",
        request_id
    );
            HttpResponse::Ok().finish()
        }
        Err(e)=>{
            tracing::error!(
        "request_id {} -- Failed to execute query: {:?}",
        request_id,
                e
    );
            HttpResponse::InternalServerError().finish()
        }
    }
}
