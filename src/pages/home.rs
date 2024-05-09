use leptos::*;
use stylers::style_sheet_str;
use crate::components::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("src/pages/styles/home.css");
    
    
    view! { class=class_name,
        
        
        <style>{style_val}</style> 
        //<img src="/assets/prudential.png" />
        <div class="bg-image"> </div>
        <div class="container"> 
            <h1 class="greeting" >"Welcome to YAPP"</h1>
        </div>
        
    }
}