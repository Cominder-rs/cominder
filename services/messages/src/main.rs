mod api;

use std::time::Duration;

use civilization::init_service;
use http::{HeaderName, HeaderValue};
use tonic::{transport::Server, Request, Status, Response};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use beijing::token_intercept;

const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
const DEFAULT_ALLOW_HEADERS: [&str; 4] =
    ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = init_service();

    let pod_ip = std::env::var("POD_IP").unwrap_or("0.0.0.0".to_string());

    let addr = pod_ip + ":80";

    let addr = addr.parse().unwrap();

    let cors = CorsLayer::new()
        .allow_origin(
            "https://www.constellation-project.ru"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true)
        .max_age(DEFAULT_MAX_AGE)
        .expose_headers(
            DEFAULT_EXPOSED_HEADERS
                .iter()
                .cloned()
                .map(HeaderName::from_static)
                .collect::<Vec<HeaderName>>(),
        )
        .allow_headers(
            DEFAULT_ALLOW_HEADERS
                .iter()
                .cloned()
                .map(HeaderName::from_static)
                .collect::<Vec<HeaderName>>(),
        );

    tracing::event!(tracing::Level::INFO, "Users app is ready!");

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .serve(addr)
        .await?;

    Ok(())
}

// fn chech_auth(req: Request<()>) -> Result<Request<()>, Status> {
//     match req.metadata().get("Authorization") {
//         Some(t) => {
//             let _token = t.to_str().map_err(|_| Status::unauthenticated(""))?;
            
//             Ok(req)
//         }
//         None => Err(Status::unauthenticated("")),
//     }
// }