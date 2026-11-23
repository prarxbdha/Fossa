use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct BrowserState {
    pub current_url: String,
}

pub type SharedState = Mutex<BrowserState>;
