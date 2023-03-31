use axum::extract::{DefaultBodyLimit, Multipart};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{any, get, post};
use axum::Router;
use fern::Dispatch;
use log::LevelFilter;
use rusvid_lib::composition::Composition;
use rusvid_lib::prelude::holder::utils::random_id;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

#[derive(Debug)]
struct SharedData {
    composition: Composition,
    id: String,
}

#[tokio::main]
async fn main() {
    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let (tx, mut rx): (UnboundedSender<SharedData>, UnboundedReceiver<SharedData>) =
        mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("{}: {:?}", message.id, message.composition);
        }
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", any(|| async { StatusCode::OK }))
        .route(
            "/upload",
            post({
                let shared_state = tx.clone();
                move |multipart| accept_form(multipart, shared_state)
            }),
        )
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

async fn accept_form(
    mut multipart: Multipart,
    tx: UnboundedSender<SharedData>,
) -> impl IntoResponse {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" {
            file = Some(data);
            break;
        }
    }

    let id = random_id();

    let out = serde_yaml::from_slice::<Composition>(&file.unwrap()).unwrap();
    tx.send(SharedData {
        composition: out,
        id: id.clone(),
    })
    .unwrap();

    // let mut renderer = EmbeddedRenderer::new("out.mp4");
    // renderer.render(out.unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::ETAG, id.parse().unwrap());

    (StatusCode::CREATED, headers)
}
