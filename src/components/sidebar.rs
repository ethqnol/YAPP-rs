#![allow(unused_imports)]
#![allow(dead_code)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stylers::style_sheet_str;

use crate::pages::HomePage;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("src/components/styles/sidebar.css");
    view! { class=class_name,
        <style>{style_val}</style>
        
        <div class="sidenav">
            <a href="/">"Progress"</a>
            <a href="/">"To Do"</a>
            <a href="/contacts">"Sources"</a>
            <a href="/contacts">"Notecards"</a>
        </div>
    }
}