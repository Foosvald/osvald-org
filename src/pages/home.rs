use yew::prelude::*;
use yew_router::prelude::*;

pub struct Home;
use crate::router::Route;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <h2>{"About me"}</h2>
            <img width=200 height=200 src="osvald_ivarsson.jpeg" />
            <p>{"I’m Osvald Ivarsson, a Swedish software developer who lives and works in Stockholm, Sweden."}</p>
            <p>{"Programming and solving problems using technology have been a big part of my life for a long time. I’m driven by quickly providing full stack solutions which solve problems and create a lot of value for the users."}</p>
            <p>{"When I’m not programming at work or on a side project I enjoy spending time with my family and friends, traveling, playing badminton, running, brewing beer or relaxing with a good book."}</p>
            <p>{"Here is my CV in "}<a href="CV_osvald_ivarsson_english.pdf">{"English [PDF]"}</a> {" and "}<a href="CV_osvald_ivarsson_svenska.pdf">{"Swedish [PDF]"}</a>{"."}</p>
            <p>{"If you want to discuss tech, jobs, or anything else, feel free to "}
                <Link<Route> to={Route::Contact}>
                  { "contact me" }
                </Link<Route>>
            {"!"}</p>
          </div>
        }
    }
}