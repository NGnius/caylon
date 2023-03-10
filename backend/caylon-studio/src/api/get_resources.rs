use actix_web::{get, web, Responder};

#[get("/{name}")]
pub async fn resource(path: web::Path<String>) -> impl Responder {
    //println!("GET resource {}", path);
    let filepath = std::path::PathBuf::from("dist").join(&*path);
    actix_files::NamedFile::open_async(filepath).await
}
