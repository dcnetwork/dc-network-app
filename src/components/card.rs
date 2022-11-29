use yew::prelude::*;
use crate::theme::Theme;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub img_path: String,
    pub name: String
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <div class="card">
            <div class="card-icon">
                <img src={format!("public/{}",props.img_path)}/>
            </div>
            <div class="card-info">
                {format!("{}",props.name)}
            </div>
        </div>
    }

}