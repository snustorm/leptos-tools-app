use leptos::*;
use leptos_router::*;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    
    
    
    view! {
        <div>
            <h2>"Parent Child Home"</h2>
            <p>"This is the parent child home page"</p>
            <ul>
                <li>
                    <a href="/parent-child/write-signal">"Write Signal"</a>
                    <a href="/parent-child/callback">"Call Back"</a>
                </li>
            </ul>
            <Outlet />
        </div>
    }
}