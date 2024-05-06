use leptos::*;
use stylers::style;
use crate::components::Nav;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let styler_class = style! { "App",
            #two{
                color: blue;
            }
            div.one{
                color: red;
                content: raw_str(r#"\hello"#);
                font: "1.3em/1.2" Arial, Helvetica, sans-serif;
            }
            div {
                border: 1px solid black;
                margin: 25px 50px 75px 100px;
                background-color: lightblue;
            }
            h1 {
                color: purple;
            }
            @media only screen and (max-width: 1000px) {
                h3 {
                    background-color: lightblue;
                    color: blue
                }
            }
        };
    view! { class = styler_class,
        <Nav/>
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}