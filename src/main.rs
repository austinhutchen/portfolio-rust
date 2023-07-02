use yew::prelude::*;
use wasm_bindgen::prelude::*;
// http://127.0.0.1:8080




#[wasm_bindgen(module = "/src/components/helpers.js")]
extern "C" {
    fn add(a: u32, b: u32) -> u32;
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <html>

        <div class={classes!("center")}>
        <h1 ><b><u>{"Hello World!"}</u></b></h1>
        <a href="" class={classes!("unify")}>{"Welcome to my page!"}</a>
        <ul class={classes!("landing")} >
       <li> <button onclick="document.location = ./views/about.html">  </button> </li>
       <li> <a href=""> <b>{"projects"} </b> </a> </li>
       <li> <a href=""> <b> {"resume"} </b> </a>  </li>
       <li> <a href=""> <b>{"more"} </b> </a>  </li>
        </ul>
        </div>
        </html>

    }
}

fn main() {
    let app = yew::Renderer::<App>::new();
    app.render();
}
