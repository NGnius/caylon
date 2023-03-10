use actix_web::{get, web, Responder};
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) static INDEX_HITS: AtomicU64 = AtomicU64::new(0);

#[get("/stats/hits")]
pub async fn hits() -> impl Responder {
    web::Json(INDEX_HITS.load(Ordering::Relaxed))
}
