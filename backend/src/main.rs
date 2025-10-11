use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use git2::{BranchType, Repository, TreeWalkMode, TreeWalkResult};
use serde::Serialize;
use serde_json::{Value, json};
use std::{env, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    repo_path: Arc<PathBuf>,
}

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
        .route("/api/files/{branch}/{path}", get(get_files))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct RepositoryInfo {
    name: String,
    main_branch: Option<String>,
}

async fn get_info(State(state): State<AppState>) -> Json<RepositoryInfo> {
    let repo = Repository::open_bare(&*state.repo_path).unwrap();
    let repo_name = repo
        .path()
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let main_branch = repo
        .head()
        .ok()
        .and_then(|r| r.resolve().ok()?.shorthand().map(|s| s.to_string()));

    Json(RepositoryInfo {
        name: repo_name.to_string(),
        main_branch,
    })
}

async fn get_branches(State(state): State<AppState>) -> Result<Json<Value>, String> {
    let repo = match Repository::open_bare(&*state.repo_path) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error opening repo: {}", e)),
    };

    let mut output: Vec<String> = Vec::new();
    if let Ok(mut branches) = repo.branches(Some(BranchType::Local)) {
        while let Some(branch) = branches.next() {
            if let Ok((b, _)) = branch {
                if let Ok(name) = b.name() {
                    if let Some(name) = name {
                        output.push(name.to_string());
                    }
                }
            }
        }
    }
    if output.is_empty() {
        Err("No branches found".into())
    } else {
        Ok(Json(json!(output)))
    }
}

#[derive(Serialize)]
struct Files {
    blobs: Vec<String>,
    trees: Vec<String>,
}

async fn get_files(
    State(state): State<AppState>,
    Path((branch, path)): Path<(String, String)>,
) -> Result<Json<Files>, String> {
    let repo = match Repository::open_bare(&*state.repo_path) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error opening repo: {}", e)),
    };
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return Err("No HEAD found".into()),
    };
    let commit = match head.peel_to_commit() {
        Ok(c) => c,
        Err(_) => return Err("Failed to read HEAD commit".into()),
    };
    let tree = match commit.tree() {
        Ok(t) => t,
        Err(_) => return Err("Failed to read tree".into()),
    };

    let mut files = Files {
        blobs: Vec::new(),
        trees: Vec::new(),
    };
    tree.walk(TreeWalkMode::PreOrder, |_, entry| {
        if let Some(name) = entry.name() {
            let name = name.to_string();
            if entry.kind() == Some(git2::ObjectType::Blob) {
                files.blobs.push(name);
            } else if entry.kind() == Some(git2::ObjectType::Tree) {
                files.trees.push(name);
            }
        }
        TreeWalkResult::Ok
    })
    .unwrap();

    Ok(Json(files))
}
