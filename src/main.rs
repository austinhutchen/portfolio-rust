use yew::prelude::*;

#[function_component(App)]
fn app()->Html{

html!{
    <body>
    <h1>{"Hello World!"}</h1>
    <b>{"this is hello world"}</b>
    </body>
   
}

}










fn main() {
    let app =     yew::Renderer::<App>::new();
    app.render();
}
