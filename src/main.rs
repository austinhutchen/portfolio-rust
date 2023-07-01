use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <body >
        <link rel="stylesheet" type="text/css" rel="noopener" target="_blank" href="./components/styles.css.css"/>
        <h1 class="test">{"Hello World!"}</h1>
        <b>{"this is hello world"}</b>
        </body>

    }
}

fn main() {
    let app = yew::Renderer::<App>::new();
    app.render();
}
