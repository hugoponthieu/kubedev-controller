use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::Data};

use crate::{config::HttpServerConfig, health::HealthState};

#[derive(Clone)]
pub struct ServerState {
    pub health: HealthState,
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/readyz")]
async fn readyz(data: Data<ServerState>) -> impl Responder {
    if data.health.is_ready() {
        HttpResponse::Ok()
    } else {
        HttpResponse::ServiceUnavailable()
    }
}

pub async fn start_server(
    config: &HttpServerConfig,
    health: HealthState,
) -> Result<(), HttpServerError> {
    HttpServer::new(move || {
        let server_state = ServerState {
            health: health.clone(),
        };
        App::new()
            .app_data(Data::new(server_state.clone()))
            .service(healthz)
            .service(readyz)
    })
    .bind((config.host.clone(), config.port))?
    .run()
    .await?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum HttpServerError {
    #[error("Could not start server: {0}")]
    Bind(#[from] std::io::Error),
}
