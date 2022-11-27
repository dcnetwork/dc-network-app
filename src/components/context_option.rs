use yew::prelude::*;
use crate::theme::Theme;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}

#[function_component(ThemedContextOption)]
pub fn themedcop() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <ContextOption {theme}/>
    }
}

#[function_component(ContextOption)]
pub fn cop(props: &Props) -> Html {

    let theme = props.theme.clone();

    html! {
        <div class="left-context" style={format!("background: {}; color: {};",theme.background,theme.foreground)}>
            <div class="left-context-item">
                <input placeholder="Search ... " style={format!("background: {}; color: {};",theme.background,theme.foreground)}/>
            </div>
        </div>
    }

}