pub mod japan;

use japan::{JapanImages, JapanPaths};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/image_svg")]
    ImageSVG,
    #[at("/path_svg")]
    PathSVG,
}

#[function_component(HomeLink)]
fn go_home() -> Html {
    html! {
        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
    }
}

#[function_component(PathSVG)]
fn path_svg() -> Html {
    let prefecture_handle = use_state(String::default);
    html! {
        <>
        <h1>{ "SVG Map" }</h1>
        <ul>
            <li><HomeLink /></li>
            <li><Link<Route> to={Route::ImageSVG}>{"Using <image> tags"}</Link<Route>></li>
        </ul>
        <h2>{"Prefecture: "} {(*prefecture_handle).to_string()}</h2>
        <JapanPaths handler={prefecture_handle}/>
        </>
    }
}

#[function_component(ImageSVG)]
fn image_svg() -> Html {
    let prefecture_handle = use_state(String::default);
    html! {
        <>
        <h1>{ "SVG Map" }</h1>
        <ul>
        <li><HomeLink /></li>
        <li><Link<Route> to={Route::PathSVG}>{"Using <path> tags"}</Link<Route>></li>
        </ul>
        <h2>{"Prefecture: "} {(*prefecture_handle).to_string()}</h2>
        <JapanImages handler={prefecture_handle}/>
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
        <h1>{ "SVG Map" }</h1>
        <ul>
        <li><a href="https://blog.whynotcats.com/svg-path">{"Blog Post"}</a></li>
        <li><Link<Route> to={Route::ImageSVG}>{"SVG with <image> tags"}</Link<Route>></li>
        <li><Link<Route> to={Route::PathSVG}>{"SVG with <path> tags"}</Link<Route>></li>
        </ul>
        <h3>{"Experiment"}</h3>
        <p>{""}</p>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::ImageSVG => html! { <ImageSVG /> },
        Route::PathSVG => html! { <PathSVG /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
