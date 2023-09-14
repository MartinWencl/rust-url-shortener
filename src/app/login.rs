use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::*;
use crate::types::{LoginInfo, LoginInfoWrapper};

/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            login(request).await
        })
    };

    use_effect_with_deps(
        move |user_login| {
            if let Some(user_info) = &user_login.data {
                user_ctx.login(user_info.user.clone());
            }
            || ()
        },
        user_login.clone(),
    );

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_login.run();
        })
    };
    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div>
            <div class="container page">
                <div>
                    <div>
                        <h1>{ "Sign In" }</h1>
                        <p class="footer-link">
                            <Link<AppRoute> to={AppRoute::Register}>
                                { "Need an account?" }
                            </Link<AppRoute>>
                        </p>
                        <ListErrors error={user_login.error.clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="input__field"
                                        type="email"
                                        placeholder="Email"
                                        value={login_info.email.clone()}
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="input__field"
                                        type="password"
                                        placeholder="Password"
                                        value={login_info.password.clone()}
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
