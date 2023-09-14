use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::components::footer::Footer;

use crate::services::user_context_provider::UserContextProvider;

use crate::app::login::Login;
use crate::app::home::Home;
use crate::app::not_found::NotFound;

pub mod home;
pub mod not_found;
pub mod login;

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

pub fn switch(route: AppRoute) -> Html {
    match route {
        | AppRoute::Home => html!{<Home />},
        | AppRoute::Login => html!{<Login />},
        | AppRoute::Register => html!{<NotFound />},
        | AppRoute::NotFound => html!{<NotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Header/>
                <Switch<AppRoute> render={switch} />
                <Footer/>
            </UserContextProvider>
        </BrowserRouter>
    }
}