use once_cell::sync::OnceCell;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Props, Clone, Default)]
pub struct Config {
    footer_message: String,
    root_path: String,
    title: String,
    search_api_endpoint: Option<String>,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            footer_message: String::new(),
            root_path: String::new(),
            search_api_endpoint: None,
            title: String::new(),
        }
    }

    #[wasm_bindgen(js_name = setFooterMessage)]
    pub fn set_footer_message(mut self, message: String) -> Self {
        self.footer_message = message;
        self
    }

    #[wasm_bindgen(js_name = getFooterMessage)]
    pub fn get_footer_message(&self) -> String {
        self.footer_message.clone()
    }

    #[wasm_bindgen(js_name = setTitle)]
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    #[wasm_bindgen(js_name = getTitle)]
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(js_name = setRootPath)]
    pub fn set_root_path(mut self, root_path: &str) -> Self {
        self.root_path = root_path.to_string();
        self
    }

    #[wasm_bindgen(js_name = getRootPath)]
    pub fn get_root_path(&self) -> String {
        self.root_path.clone()
    }

    #[wasm_bindgen(js_name = getSearchApiEndpoint)]
    pub fn get_search_api_endpoint(&self) -> String {
        self.search_api_endpoint.clone().unwrap_or_default()
    }

    #[wasm_bindgen(js_name = setSearchApiEndpoint)]
    pub fn set_search_api_endpoint(mut self, url: &str) -> Self {
        if !url.is_empty() {
            self.search_api_endpoint = Some(url.to_string());
        }

        self
    }

    pub fn is_enable_search(&self) -> bool {
        self.search_api_endpoint.is_some()
    }
}

pub static APP_OPTIONS: OnceCell<Config> = OnceCell::new();
