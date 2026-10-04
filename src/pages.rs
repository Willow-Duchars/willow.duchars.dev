use alloy_calculator::AlloyCalculator;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use transition_timer::TransitionTimer;
use window_lib::prelude::*;

mod alloy_calculator;
mod transition_timer;

#[derive(PartialEq, Eq)]
enum Page {
    About,
    AlloyCalculator,
    InvalidPath,
    TransitionTimer,
}

#[component]
pub fn pages() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=InvalidPath>
                <Route path=path!("") view=|| view! { <OpenPage open=Page::About/> }/>
                <Route path=path!("/about") view=|| view! { <OpenPage open=Page::About/> }/>
                <Route
                    path=path!("/alloy-calculator")
                    view=|| view! { <OpenPage open=Page::AlloyCalculator/> }
                />
                <Route
                    path=path!("/transition-timer")
                    view=|| view! { <OpenPage open=Page::TransitionTimer/> }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn open_page(open: Page) -> impl IntoView {
    view! {
        <About is_open=open == Page::About/>
        <AlloyCalculator is_open=open == Page::AlloyCalculator/>
        <TransitionTimer is_open=open == Page::TransitionTimer/>
    }
}

#[component]
pub fn about(is_open: bool) -> impl IntoView {
    let data = WindowData::new(icons::ABOUT, "About")
        .centered()
        .open(is_open);
    view! {
        <Window data>
            <h1>"Yo waddup!"</h1>
            <p>
                "Hi, I'm Willow Duchars, a recent computer science graduate from Lindenwood Univsersity.
                Some of my hobbies include playing and theorizing about Magic: The Gathering, seeking out 
                all the achievements in various videogames, collecting soundtracks, and working on personal 
                coding projects such as this one."
            </p>
        </Window>
    }
}

#[component]
pub fn invalid_path() -> impl IntoView {
    let data = WindowData::new(icons::CLOSE, "Invalid Path")
        .centered()
        .create_desktop_item(false);
    view! {
        <OpenPage open=Page::InvalidPath/>
        <Window data>
            <h1 style="text-align:center;">"Oops"</h1>
            <p style="text-align:center;">
                "It seems the path you were trying to reach doesn't exist."
            </p>
        </Window>
    }
}
