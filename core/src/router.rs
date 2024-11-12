use gloo::history::{HashHistory, History};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SearchParams {
    pub doc: Option<String>,
    pub sidebar: Option<String>,
}

#[derive(Clone)]
pub struct Router {
    hash_history: HashHistory,
}

impl Router {
    pub fn new() -> Self {
        Self {
            hash_history: HashHistory::new(),
        }
    }

    pub fn route(&self) -> Result<(String, SearchParams), Box<dyn std::error::Error>> {
        let location = self.hash_history.location();
        let path = location.path();
        let search_params = location.query::<SearchParams>()?;

        Ok((path.to_string(), search_params))
    }
}
