use yew::prelude::*;
use web_sys::window;
use web_sys::*;
use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use js_sys::Function;

use crate::theme::Theme;


#[derive(PartialEq, Properties)]
pub struct Props {
    theme: Theme,
}
//
pub struct Navbar;

impl Component for Navbar{

    type Message = ();

    type Properties = Props;

    fn create( _ctx: &Context<Self> )-> Self{
        Self
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        
    }

    // The VIEW
    fn view(&self, ctx:&Context<Self>) -> Html {

        let theme = &ctx.props().theme;

		html!{
			<main class="fullcontent" style={format!("background: {}; color: {};",theme.background,theme.foreground)}>
                <div class="nav-container">
                    <div class="nav-item-top">
                        <div class="nav-item">
                            <a>
                                <img src="public/home.svg" class="logo tauri" alt="Home"/>
                            </a>
                        </div>
                        <div class="nav-item">
                            <a>
                                <img src="public/message.svg" class="logo tauri" alt="Messages"/>
                            </a>
                        </div>
                    </div>
                    <div class="nav-item-below">
                        <div class="nav-item">
                            <a>
                                <img src="public/settings.svg" class="logo tauri" alt="settings"/>
                            </a>
                        </div>
                    </div>
                </div>
            </main>
		}        
    }
}

#[function_component(ThemedNavbar)]
pub fn themednavbar() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <Navbar {theme}/>
    }
}