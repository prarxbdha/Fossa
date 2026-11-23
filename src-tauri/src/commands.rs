use fossa_engine::network::fetch::fetch;
use fossa_engine::paint::display_list::{DisplayItem, DisplayList};
use serde::Serialize;
use tauri::State;

use crate::state::SharedState;

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DisplayItemDto {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [u8; 4],
    },
    Text {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: String,
        color: [u8; 4],
        font_size: f32,
    },
}

#[derive(Serialize)]
pub struct DisplayListDto {
    items: Vec<DisplayItemDto>,
}

impl From<DisplayList> for DisplayListDto {
    fn from(list: DisplayList) -> Self {
        Self {
            items: list.items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DisplayItem> for DisplayItemDto {
    fn from(item: DisplayItem) -> Self {
        match item {
            DisplayItem::Rect {
                x,
                y,
                width,
                height,
                color,
            } => Self::Rect {
                x,
                y,
                width,
                height,
                color: [color.r, color.g, color.b, color.a],
            },
            DisplayItem::Text {
                x,
                y,
                width,
                height,
                text,
                color,
                font_size,
            } => Self::Text {
                x,
                y,
                width,
                height,
                text,
                color: [color.r, color.g, color.b, color.a],
                font_size,
            },
        }
    }
}

#[tauri::command]
pub fn get_current_url(state: State<'_, SharedState>) -> String {
    state
        .lock()
        .map(|state| state.current_url.clone())
        .unwrap_or_default()
}

#[tauri::command]
pub fn set_current_url(state: State<'_, SharedState>, url: String) {
    if let Ok(mut state) = state.lock() {
        state.current_url = url;
    }
}

#[tauri::command]
pub fn fetch_document(url: String) -> Result<String, String> {
    fetch(&url).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn read_fixture(name: String) -> Result<String, String> {
    match name.as_str() {
        "basic" => Ok(include_str!("../../fixtures/basic.html").to_owned()),
        "typography" => Ok(include_str!("../../fixtures/typography.html").to_owned()),
        "styles" => Ok(include_str!("../../fixtures/styles.html").to_owned()),
        "links" => Ok(include_str!("../../fixtures/links.html").to_owned()),
        other => Err(format!("unknown built-in page: {other}")),
    }
}

#[tauri::command]
pub fn render_document(html: String, viewport_width: f32) -> DisplayListDto {
    let dom = fossa_engine::html::parser::parse(&html);
    let layout = fossa_engine::layout::engine::layout(&dom, viewport_width);
    let display_list = fossa_engine::paint::painter::paint(&layout);
    display_list.into()
}
