use wasm_bindgen::prelude::*;
use sycamore::prelude::*;

#[wasm_bindgen]
#[derive(Prop, Clone)]
pub struct Config {
    footer_message: String,
    root_path: String,
    title: String,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            footer_message: String::new(),
            root_path: String::new(),
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
    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    #[wasm_bindgen(js_name = getTitle)]
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(js_name = setRootPath)]
    pub fn set_root_path(mut self, root_path: String) -> Self {
        self.root_path = root_path;
        self
    }

    #[wasm_bindgen(js_name = getRootPath)]
    pub fn get_root_path(&self) -> String {
        self.root_path.clone()
    }
}
