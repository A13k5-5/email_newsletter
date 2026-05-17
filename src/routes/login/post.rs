use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header(("LOCATION", "/"))
        .finish()
}
