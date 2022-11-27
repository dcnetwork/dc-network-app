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

            </div>
            <div class="right-context-main">
            </div>
        </div>
    }

}