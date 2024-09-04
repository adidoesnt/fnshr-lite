use actix_web::HttpResponse;

pub fn get_health_status() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
