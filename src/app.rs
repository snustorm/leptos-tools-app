use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::parent_child::parent_child_home::ParentChildHome;
use crate::components::parent_child::parent_child_write_signal::ParentChildWriteSignal;
use crate::components::parent_child::parent_child_callback::ParentCallBack;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! {
        <div class="container">
            {children()}
        </div>
    }
}

#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
        <header id="page-header" class="page-header">
            <h1>"Tools App"</h1>
        </header>
    }
}

#[component]
pub fn PageFooter() -> impl IntoView {
    view! {
        <footer id="page-footer" class="page-footer">
            <p>"© 2021 Tools App"</p>
        </footer>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav id="main-menu">
            <ul>
                <li class="menu-item">
                    <a href="/">"Home"</a>
                </li>
                <li class="menu-item">
                    <a href="/parent-child">"Parent Child"</a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! { <aside id="sidebar">Sidebar</aside> }
}

#[component]
pub fn Content() -> impl IntoView {
    view! { 
        <Router>
            <main id="content">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/parent-child" view=ParentChildHome>
                        <Route path="write-signal" view=ParentChildWriteSignal/ > 
                        <Route path="callback" view=ParentCallBack/ > 
                        <Route path="" view=|| view! { <p>"Click an example link" </p>} / > 
                    </Route>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
     }
}


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tools-app.css"/>

        // sets the document title
        <Title text="Welcome to Tools App"/>

        <Container>
            <PageHeader />
            <NavBar />
            <SideBar />
            <Content />
            <PageFooter />

        </Container>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
