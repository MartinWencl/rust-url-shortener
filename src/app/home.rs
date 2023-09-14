use yew::prelude::*;

#[function_component(Home)]
pub fn footer() -> Html {
    html!{
        <div class="content-container">
            <p>{ "Welcome to the URL shortener by martinw." }</p>
        </div>
    }
}