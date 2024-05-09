#![allow(unused_imports)]
#![allow(dead_code)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stylers::style_sheet_str;

use crate::pages::HomePage;

#[component]
pub fn Nav() -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("src/components/styles/nav.css");
    view! { class=class_name,
        <style>{style_val}</style>
        
        <nav class="navbar">
            <h2 class="website-name"> Yet Another Paper Planner </h2>
            
            <div class="main-navigation"> 
                <a href="/">"Home"</a>
                <a href="/project">"Project"</a>
            </div>

        </nav>
        
    }
}