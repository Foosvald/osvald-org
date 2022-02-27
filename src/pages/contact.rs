use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
   html! {
      <div>
        <h1>{"Contact"}</h1>
        <p>{"If you want to get in touch with me send an email to "} <a href="mailto:osvald@osvald.org">{"osvald@osvald.org"}</a>
        {" or contact me on "}
        <a href="https://www.linkedin.com/in/osvaldivarsson">{"LinkedIn"}</a>{"."}
        </p>
      </div>
   }
}