use leptos::*;
use stylers::style_sheet_str;
use crate::components::Nav;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let (class_name, style_val) = style_sheet_str!("src/pages/styles/home.css");
    
    
    view! { class=class_name,
        <style>{style_val}</style>
        
        
        
        <Nav/>
        
        <h1 class="hi" >"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}