extern crate yew;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod router;
mod components;
use components::navbar::Navbar;
use components::footer::Footer;

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
                <div class="wrapper">
                    <Navbar />
                    <div class="content">
                        <Switch<router::Route> render={Switch::render(router::switch)} />
                    </div>
                    <Footer />
                </div>
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}