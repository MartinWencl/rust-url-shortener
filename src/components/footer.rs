use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html!{
        <div class="footer">
            <div>
                { contact_links() }
            </div>
        </div>
    }
}

pub fn contact_links() -> Html {
    const GITHUB_REF: &str = "https://github.com/MartinWencl";
    const LINKEDIN_REF: &str = "https://www.linkedin.com/in/martin-wencl-423b33241/";
    html!{
        <ul class="nav-item">
            <li class="footer-link">
                <a href={GITHUB_REF}>{ "My GitHub" }</a>
            </li>
            <li class="footer-link">
                <a href={LINKEDIN_REF}>{ "My LinkedIn" }</a>
            </li>
        </ul> 
    }
}