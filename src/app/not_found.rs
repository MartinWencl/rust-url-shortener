use yew::prelude::*;

#[function_component(NotFound)]
pub fn footer() -> Html {
    html!{
        <div class="content-container">
            <p>{ "🚧🚨 Sorry, this site was not found 🚨🚧" }</p>
        </div>
    }
}