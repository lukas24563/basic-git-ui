use axum::{Json, extract::State};
use git2::Repository;
use serde_json::{Value, json};

use crate::common::AppState;

pub async fn get_branches(State(state): State<AppState>) -> Result<Json<Value>, String> {
    let repo = Repository::open_bare(&*state.repo_path)
        .map_err(|e| format!("Error opening repo: {}", e))?;

    let branches = repo
        .branches(None)
        .map(|iter: git2::Branches<'_>| {
            iter.filter_map(Result::ok)
                .filter_map(|(branch, _)| branch.name().ok().flatten().map(|name| name.to_string()))
                .collect::<Vec<_>>()
        })
        .map_err(|e| format!("Error accessing branches: {}", e))?;

    Ok(Json(json!(branches)))
}
