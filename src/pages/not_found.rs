use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
   let history = use_history().unwrap();

   let go_back = Callback::from(move |_| history.back());
   html! {
      <div>
         <h1>{"404 - Not found"}</h1>
         <p>{"This is not good. Please do "}
         <Link<Route> to={Route::Contact}>
           { "tell me" }
         </Link<Route>>
         {" if you think this was supposed to be here..."}</p>
         <button onclick={go_back}>{"Go back"}</button>
      </div>
   }
}