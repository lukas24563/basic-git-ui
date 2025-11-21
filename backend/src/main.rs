use axum::{Router, routing::get};
use basic_git_ui::branches::get_branches;
use basic_git_ui::info::get_info;
use basic_git_ui::tree::tree_routes;
use basic_git_ui::{blob::blob_routes, common::AppState};
use git2::Repository;
use std::{env, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected one argument, got {}", args.len() - 1);
        std::process::exit(1);
    }

    let repo_path = PathBuf::from(&args[1]);
    if Repository::open_bare(&repo_path).is_err() {
        eprintln!(
            "There is no bare git repository at: {}",
            repo_path.display()
        );
        std::process::exit(1);
    }

    println!(
        "Hosting backend for git repository in path {}",
        repo_path.display()
    );

    let state = AppState {
        repo_path: Arc::new(repo_path),
    };

    let app = Router::new()
        .route("/api/info", get(get_info))
        .route("/api/branches", get(get_branches))
        .nest("/api/tree", tree_routes())
        .nest("/api/blob/", blob_routes())
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
