use leptos::*;
use stylers::style_sheet_str;
use crate::components::*;

#[component]
pub fn Project() -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("src/pages/styles/project.css");
    
    
    view! { class=class_name,
        
        <style>{style_val}</style> 
        <Sidebar />
        <div class="content-container"> 
            <h1 class="greeting" >"Welcome to YAPP"</h1>
        </div>
        
    }
}