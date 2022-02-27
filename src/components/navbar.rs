use yew::prelude::*;
use yew_router::prelude::*;


use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
   html! {
      <div class="navbar-start">
         <div>{"Osvald Ivarsson"}</div>
        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
          { "Home" }
        </Link<Route>>
        <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
          { "Contact" }
        </Link<Route>>
      </div>
   }
}
