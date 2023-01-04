use leptos::*;
use tauri_glue::*;

#[tauri_glue::bind_command(name = hello)]
pub async fn hello(name: Option<String>) -> Result<JsValue, JsValue>;

#[component]
pub fn Sidebar(cx: Scope) -> Element {
    let (sidebar_visible, set_sidebar_visibility) = create_signal(cx, true);
    let toggle_sidebar = move |_| set_sidebar_visibility.update(|v| *v = !*v);

    view! {
      cx,
      <div class:sidebar=true class:sidebar-hidden=sidebar_visible>
        <h1 class:sidebar-title=true>"Journal"</h1>
        <div on:click=toggle_sidebar class:sidebar-hide-btn=true>
          <div class:close-btn=true />
        </div>
      </div>
    }
}
