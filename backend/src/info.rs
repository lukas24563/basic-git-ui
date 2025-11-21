use axum::{Json, extract::State};
use git2::Repository;
use serde::Serialize;

use crate::common::AppState;

#[derive(Serialize)]
pub struct RepositoryInfo {
    name: String,
    main_branch: Option<String>,
}

pub async fn get_info(State(state): State<AppState>) -> Result<Json<RepositoryInfo>, String> {
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
