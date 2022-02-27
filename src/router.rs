use yew::prelude::*;
use yew_router::prelude::*;

use super::pages::{home::Home, contact::Contact, not_found::NotFound};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Contact => html! {<Contact/>},
        Route::NotFound => html! {<NotFound/>}
    }
}