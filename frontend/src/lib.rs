#![recursion_limit="512"]
use yew::App;
use wasm_bindgen::prelude::*;
use yew::{ShouldRender,Html,html,ComponentLink,Component,Properties,Callback};
mod project;
mod projectList;
mod error;
mod service;
mod todo;
use models;
use projectList::ProjectListComponent;
use console_log;
use dotenv;
use log::{info,debug};


#[wasm_bindgen(start)]
pub fn run_app() {
    console_log::init().expect("error initializing logger");
    info!("initialized app");
    dotenv::dotenv().ok();

    App::<Wrapper>::new().mount_to_body();
}

struct Wrapper {
    link: ComponentLink<Self>
}

impl Component for Wrapper {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>)  -> Self {
        Self{link}
    }

    fn update(&mut self, _msg:Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div>
                {"wrapper"}
                <ProjectListComponent/> 
            </div>
        }
    }
}
