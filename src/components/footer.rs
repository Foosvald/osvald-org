use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
   html! {
      <div class="footer">
         <hr/>
         <div>{"Made by Osvald Ivarsson in Rust and WebAssembly (totally overkill for this small site though). See source here: "}<a href="https://github.com/Foosvald/osvald-org">{"https://github.com/Foosvald/osvald-org"}</a></div>
      </div>
   }
}
