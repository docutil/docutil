use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Route {
    pub doc: Option<String>,
    pub sidebar: Option<String>,
}

pub fn get_route() -> anyhow::Result<Route> {
    let window = web_sys::window().unwrap();
    let location = &window.location();

    let search_str = location.search().unwrap_or("".to_string());

    let route = serde_urlencoded::from_str::<Route>(search_str.strip_prefix('?').unwrap_or(""))?;

    Ok(route)
}
