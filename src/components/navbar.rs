use yew::prelude::*;
use yew_router::prelude::*;


use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
   html! {
      <nav class="navbar">
         <ul>
            <li class="header">
               <h1>{"Osvald Ivarsson"}</h1>
            </li>
            <li>
               <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                 { "Home" }
               </Link<Route>>
            </li>
            <li>
               <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                 { "Contact" }
               </Link<Route>>
            </li>
         </ul>
         <hr/>
      </nav>
   }
}
