use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub repo_path: Arc<PathBuf>,
}
