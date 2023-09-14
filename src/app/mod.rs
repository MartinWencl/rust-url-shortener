use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::components::footer::Footer;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,    
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header/>
                <div class="container">
                    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. In suscipit nulla quis felis blandit lobortis. Vivamus id risus non elit vestibulum vulputate. Pellentesque quis facilisis est. Proin mi felis, faucibus lacinia congue ac, varius et mauris. Phasellus et consequat magna, id molestie massa. Nam eu tellus vitae libero maximus luctus. Integer dictum ac lacus vitae hendrerit. Suspendisse sit amet diam interdum, hendrerit quam non, blandit magna. Quisque et bibendum ex. Nulla a tellus egestas, feugiat justo sit amet, finibus orci. Vivamus vitae elementum massa, a venenatis est. Pellentesque sem tortor, vehicula in ex vitae, consequat finibus nibh. Donec ut ligula laoreet, aliquet lorem a, elementum purus. In hac habitasse platea dictumst. Integer a est venenatis, consectetur purus eu, aliquam arcu." }</p>
                </div>
            <Footer/>
        </BrowserRouter>
    }
}