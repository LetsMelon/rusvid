use axum::extract::{DefaultBodyLimit, Multipart};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use fern::Dispatch;
use log::LevelFilter;
use rusvid_lib::composition::Composition;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }).post(accept_form))
        .layer(
            ServiceBuilder::new()
                .layer(DefaultBodyLimit::disable())
                .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024))
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin("*".parse::<HeaderValue>().unwrap())
                        .allow_methods([Method::GET, Method::POST]),
                )
                .layer(CompressionLayer::new()),
        )
        .fallback(|| async { StatusCode::NOT_FOUND });

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn accept_form(mut multipart: Multipart) -> impl IntoResponse {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" {
            file = Some(data);
            break;
        }
    }

    let out = serde_yaml::from_slice::<Composition>(&file.unwrap());
    println!("{out:?}");
    // let mut renderer = EmbeddedRenderer::new("out.mp4");
    // renderer.render(out.unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::ETAG, "random_id".parse().unwrap());

    (StatusCode::CREATED, headers)
}
