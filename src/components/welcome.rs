use yew::prelude::*;


#[function_component(Welcome)]
pub fn welcome() -> Html {

    html! {
        <main >
            <div class="center-container">
                <h1>{"WELCO"}<span style={"color:#ff00ff;text-shadow: 2px 2px 20px #ff05ff;
"}>{"M"}</span><span style={"color:cyan;text-shadow: 2px 2px 20px cyan;"}>{"E"}</span></h1>
            </div>
            <p class="stat-right">{"LOADING "}<span style={"font-weight:800;animation:fade 1s infinite reverse;animation-delay:0.5s"}>{"."}</span><span style={"font-weight:800;animation:fade 1s infinite reverse;animation-delay:0.7s"}>{"."}</span><span style={"font-weight:800;animation:fade 1s infinite reverse;animation-delay:1s"}>{"."}</span></p>
        </main>
    }

}