use actix_web::HttpResponse;

pub fn health_controller() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}