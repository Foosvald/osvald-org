extern crate yew;

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod router;
mod components;
use components::navbar::Navbar;

struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Got message: {:?}", msg);
        false
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Navbar />
                <Switch<router::Route> render={Switch::render(router::switch)} />
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}