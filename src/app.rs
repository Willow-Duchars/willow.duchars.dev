use crate::pages::Pages;
use leptos::prelude::*;
use window_lib::components::{Desktop, Taskbar};
use window_lib::{DesktopItems, TaskbarItems, Windows, browser};

#[component]
pub fn app() -> impl IntoView {
    browser::setup_browser_center_listener();
    browser::setup_browser_dimensions_listener();
    let styling = browser::setup_taskbar_styling();

    provide_context(DesktopItems::default());
    provide_context(TaskbarItems::default());
    provide_context(Windows::default());

    view! {
        <style>{styling}</style>
        <main>
            <Desktop/>
            <Pages/>
        </main>
        <footer>
            <Taskbar/>
        </footer>
    }
}
