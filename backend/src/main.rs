use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use git2::{BranchType, Commit, ObjectType, Repository};
use serde::Serialize;
use serde_json::{Value, json};
use std::path::Path as FsPath;
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
        .route("/api/tree/{branch}", get(get_root_tree))
        .route("/api/tree/{branch}/{*path}", get(get_tree))
        .route("/api/blob/{branch}/{*path}", get(get_blob))
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

async fn get_info(State(state): State<AppState>) -> Result<Json<RepositoryInfo>, String> {
    let repo = Repository::open_bare(&*state.repo_path)
        .map_err(|e| format!("Error opening repo: {}", e))?;

    let repo_name = repo
        .path()
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let main_branch = repo
        .head()
        .ok()
        .and_then(|r| r.resolve().ok()?.shorthand().map(|s| s.to_string()));

    Ok(Json(RepositoryInfo {
        name: repo_name.to_string(),
        main_branch,
    }))
}

async fn get_branches(State(state): State<AppState>) -> Result<Json<Value>, String> {
    let repo = Repository::open_bare(&*state.repo_path)
        .map_err(|e| format!("Error opening repo: {}", e))?;

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

async fn get_root_tree(
    State(state): State<AppState>,
    Path(branch): Path<String>,
) -> Result<Json<Files>, String> {
    get_tree(State(state), Path((branch, String::new()))).await
}

#[derive(Serialize)]
struct Files {
    blobs: Vec<FileInfo>,
    trees: Vec<FileInfo>,
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    last_commit_message: String,
    last_commit_timestamp: String,
    last_commit_id: String,
}

fn last_commit_for_path(
    repo: &Repository,
    branch_commit: &Commit,
    path: &FsPath,
) -> Option<(String, String, String)> {
    let mut revwalk = repo.revwalk().ok()?;
    revwalk.push(branch_commit.id()).ok()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL).ok()?;
    revwalk.simplify_first_parent().ok()?;

    for oid_result in revwalk {
        let oid = oid_result.ok()?;
        let commit = repo.find_commit(oid).ok()?;

        let tree = commit.tree().ok()?;
        let parent_tree = commit.parents().next().and_then(|p| p.tree().ok());

        let diff = repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)
            .ok()?;

        for delta in diff.deltas() {
            if delta
                .new_file()
                .path()
                .or(delta.old_file().path())
                .map(|p| p.starts_with(path))
                .unwrap_or(false)
            {
                let message = commit.message().unwrap_or("").to_string();
                let timestamp = commit.time().seconds().to_string();
                let oid_str = oid.to_string();
                return Some((message, timestamp, oid_str));
            }
        }
    }

    None
}

async fn get_tree(
    State(state): State<AppState>,
    Path((branch, path)): Path<(String, String)>,
) -> Result<Json<Files>, String> {
    let repo = Repository::open_bare(&*state.repo_path)
        .map_err(|e| format!("Error opening repo: {}", e))?;

    let branch_ref = format!("refs/heads/{}", branch);
    let reference = repo
        .find_reference(&branch_ref)
        .map_err(|_| format!("Branch '{}' not found", branch))?;

    let commit = reference
        .peel_to_commit()
        .map_err(|_| format!("Failed to resolve commit for '{}'", branch))?;

    let tree = commit
        .tree()
        .map_err(|_| "Failed to read commit tree".to_string())?;

    let target_tree = if !path.is_empty() && path != "/" {
        match tree.get_path(FsPath::new(&path)) {
            Ok(entry) => {
                let obj = entry
                    .to_object(&repo)
                    .map_err(|_| format!("Failed to load object at '{}'", path))?;
                obj.peel_to_tree()
                    .map_err(|_| format!("Path '{}' is not a tree", path))?
            }
            Err(_) => return Err(format!("Path '{}' not found in tree", path)),
        }
    } else {
        tree
    };

    let mut files = Files {
        blobs: Vec::new(),
        trees: Vec::new(),
    };

    for entry in target_tree.iter() {
        if let Some(name) = entry.name() {
            let file_path = if path.is_empty() || path == "/" {
                FsPath::new(name).to_path_buf()
            } else {
                FsPath::new(&path).join(name)
            };

            let (message, timestamp, oid) = last_commit_for_path(&repo, &commit, &file_path)
                .ok_or(format!("No commit found for file: {}", file_path.display()))?;

            let info = FileInfo {
                name: name.to_string(),
                last_commit_message: message,
                last_commit_timestamp: timestamp,
                last_commit_id: oid,
            };

            match entry.kind() {
                Some(ObjectType::Blob) => files.blobs.push(info),
                Some(ObjectType::Tree) => files.trees.push(info),
                _ => {}
            }
        }
    }

    Ok(Json(files))
}

async fn get_blob(
    State(state): State<AppState>,
    Path((branch, path)): Path<(String, String)>,
) -> Result<String, String> {
    let repo = Repository::open_bare(&*state.repo_path)
        .map_err(|e| format!("Error opening repo: {}", e))?;

    let branch_ref = format!("refs/heads/{}", branch);
    let reference = repo
        .find_reference(&branch_ref)
        .map_err(|_| format!("Branch '{}' not found", branch))?;

    let commit = reference
        .peel_to_commit()
        .map_err(|_| format!("Failed to resolve commit for '{}'", branch))?;

    let tree = commit
        .tree()
        .map_err(|e| format!("Failed to get tree: {}", e))?;

    let entry = tree
        .get_path(std::path::Path::new(&path))
        .map_err(|_| format!("File '{}' not found in branch '{}'", path, branch))?;

    let blob = repo
        .find_blob(entry.id())
        .map_err(|e| format!("Failed to read blob: {}", e))?;

    let content =
        str::from_utf8(blob.content()).map_err(|_| "File is not valid UTF-8".to_string())?;

    Ok(content.to_string())
}
