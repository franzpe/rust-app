use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
        get,
        path = "/",
        responses((status = 200, description = "Hello workdl")),
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path(
        post,
        path = "/echo",
        request_body=String,
        responses((status = 200, description = "Echo response")),
)]
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(hello, echo), info(description = "My Api description"))]
    struct ApiDoc;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
