use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use git2::{Repository, Signature};
use serde::Deserialize;

use crate::common::AppState;

pub fn blob_routes() -> Router<AppState> {
    Router::new()
        .route("/{branch}/{*path}", get(get_blob))
        .route("/{branch}/{*path}", post(update_blob))
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

#[derive(Deserialize)]
struct UpdateBlobRequest {
    content: String,
    message: String,
    name: String,
    email: String,
}

async fn update_blob(
    State(state): State<AppState>,
    Path((branch, path)): Path<(String, String)>,
    Json(payload): Json<UpdateBlobRequest>,
) -> Result<String, String> {
    let repo = Repository::open_bare(&*state.repo_path)
        .map_err(|e| format!("Error opening repo: {}", e))?;

    let branch_ref = format!("refs/heads/{}", branch);
    let reference = repo
        .find_reference(&branch_ref)
        .map_err(|_| format!("Branch '{}' not found", branch))?;

    let parent_commit = reference
        .peel_to_commit()
        .map_err(|_| format!("Failed to resolve commit for '{}'", branch))?;

    let parent_tree = parent_commit
        .tree()
        .map_err(|e| format!("Failed to get tree: {}", e))?;

    let blob_oid = repo
        .blob(payload.content.as_bytes())
        .map_err(|e| format!("Failed to create blob: {}", e))?;

    let mut tree_builder = repo
        .treebuilder(Some(&parent_tree))
        .map_err(|e| format!("Failed to create tree builder: {}", e))?;

    tree_builder
        .insert(&path, blob_oid, 0o100644)
        .map_err(|e| format!("Failed to insert blob into tree: {}", e))?;

    let new_tree_oid = tree_builder
        .write()
        .map_err(|e| format!("Failed to write tree: {}", e))?;

    let new_tree = repo
        .find_tree(new_tree_oid)
        .map_err(|e| format!("Failed to find new tree: {}", e))?;

    let sig = Signature::now(&payload.name, &payload.email)
        .map_err(|e| format!("Failed to create signature: {}", e))?;

    let commit_oid = repo
        .commit(
            Some(&branch_ref),
            &sig,
            &sig,
            &payload.message,
            &new_tree,
            &[&parent_commit],
        )
        .map_err(|e| format!("Failed to create commit: {}", e))?;

    Ok(format!(
        "Updated '{}' in branch '{}' with commit {}",
        path, branch, commit_oid
    ))
}
