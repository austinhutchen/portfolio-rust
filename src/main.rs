use yew::prelude::*;
// http://127.0.0.1:8080
#[function_component(App)]
fn app() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();


    html! {
        <html>

        <div class={classes!("center")}>
        <h1 class={classes!("heading")} ><b><u>{"Hello World!"}</u></b></h1>
        <a href="" class={classes!("unify")}>{"Welcome to my page!"}</a>
        <ul class={classes!("landing")} >

        </ul>
        </div>
        </html>

    }
}

fn main() {
    let app = yew::Renderer::<App>::new();
    app.render();
}
