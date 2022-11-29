use yew::prelude::*;
use crate::theme::Theme;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}

#[function_component(ThemedContext)]
pub fn themedcop() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <Context {theme}/>
    }
}

#[function_component(Context)]
pub fn cop(props: &Props) -> Html {

    let theme = props.theme.clone();

    html! {
        <div class="right-context" style={format!("background: {}; color: {};",theme.background,theme.foreground)}>
            <div class="right-context-navbar">
                <div class="right-context-icon">
                    <img src="public/account-light.svg"/>
                </div>
                <div class="right-context-name">
                    <p>{"Vito Scaletta"}</p>
                </div>
            </div>
            <div class="right-context-main">

                <div class="context-message-input">
                    <input placeholder={"Message"}/>
                    <span>{" "}</span>
                    <button>{"+"}</button>
                </div>
            </div>
        </div>
    }

}