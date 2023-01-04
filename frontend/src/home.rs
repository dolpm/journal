use crate::sidebar::*;
use leptos::*;
use tauri_glue::*;

#[tauri_glue::bind_command(name = hello)]
pub async fn hello(name: Option<String>) -> Result<JsValue, JsValue>;

#[component]
pub fn Home(cx: Scope) -> Element {
    view! {
      cx,
      <div class:container=true>
        <Sidebar />
        <div class:notebook=true>
        </div>
      </div>
    }
}
