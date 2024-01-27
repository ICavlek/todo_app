use super::content_loader::read_file;
use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    let html_data = read_file("./templates/main.html");
    let javascript_data = read_file("./javascript/main.js");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data.replace("{{JAVASCRIPT}}", &javascript_data))
}
