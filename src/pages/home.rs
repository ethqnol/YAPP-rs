use leptos::*;
use stylers::style_sheet_str;
use crate::components::Nav;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let (class_name, style_val) = style_sheet_str!("src/pages/styles/home.css");
    
    
    view! { class=class_name,
        <Nav/>
        <style>{style_val}</style>
        
       
        
        <h1 class="greeting" >"Welcome to YAPP"</h1>
        <h1 class="get-started" >"Get Started"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}