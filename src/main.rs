#![allow(clippy::multiple_crate_versions)]
use axum::Router;
use tokio::{net::TcpListener, signal};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use website::{handler::init_routes, settings::Settings, AppState};

#[tokio::main]
async fn main() {
    // load the configuration
    let (settings, origin) = match Settings::load() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("ERROR: failed to load settings");
            eprintln!("ERROR: {e}");
            return;
        }
    };

    // initialize the logger
    tracing_subscriber::registry()
        .with(
            match tracing_subscriber::EnvFilter::try_new(&settings.log) {
                Ok(filter) => filter,
                Err(e) => {
                    eprintln!("ERROR: failed to create log filter");
                    eprintln!("ERROR: {e}");
                    return;
                }
            },
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!(r#"configuration loaded from "{}""#, origin);

    // initialize the application state
    let state = match AppState::try_from(&settings) {
        Ok(state) => state,
        Err(e) => {
            tracing::error!("failed to create application state");
            tracing::error!("{e}");
            return;
        }
    };

    // build the application with routes and middleware
    let app = Router::new()
        .merge(init_assets(&settings))
        .merge(init_routes(state))
        .layer(TraceLayer::new_for_http());

    // set up the listener
    let listen = format!("{}:{}", settings.host, settings.port);
    let listener = match TcpListener::bind(&listen).await {
        Ok(listener) => {
            tracing::info!("listening on {}", listener.local_addr().unwrap());
            listener
        }
        Err(e) => {
            tracing::error!("failed to bind to address: {}", listen);
            tracing::error!("{e}");
            return;
        }
    };

    // start the server
    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        tracing::error!("server error");
        tracing::error!("{err}");
    }
}

/// Initialize the assets for the application by creating a router that serves
/// the static files from the specified paths.
fn init_assets(settings: &Settings) -> Router {
    let mut router = Router::new();

    for asset in settings.assets.iter() {
        tracing::info!(
            r#"mounting asset path {:?} on "{}""#,
            asset.path,
            asset.route
        );
        let serve = ServeDir::new(asset.path.clone());
        router = router.nest_service(&asset.route, serve);
    }
    router
}

/// Create a future that resolves when a shutdown signal is received to allow
/// for graceful shutdown of the server.
#[allow(clippy::redundant_pub_crate)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
