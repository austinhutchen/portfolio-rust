use yew::prelude::*;
// http://127.0.0.1:8080
#[function_component(App)]
fn app() -> Html {
    html! {
        <html>

        <div >
        <h1 class={classes!("test")}>{"Hello World!"}</h1>
        <h4><b>{"this is hello world"}</b></h4>
        </div>
        </html>

    }
}

fn main() {
    let app = yew::Renderer::<App>::new();
    app.render();
}
