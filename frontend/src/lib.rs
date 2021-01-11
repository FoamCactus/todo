#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::App;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
mod error;
mod project;
mod project_list;
mod service;
mod todo;
mod todo_list;
use console_log;
use dotenv;
use log::{debug, info};
use models;
use project_list::ProjectListComponent;

#[wasm_bindgen(start)]
pub fn run_app() {
    console_log::init().expect("error initializing logger");
    info!("initialized app");
    dotenv::dotenv().ok();

    App::<Wrapper>::new().mount_to_body();
}

struct Wrapper {
    link: ComponentLink<Self>,
}

impl Component for Wrapper {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {"wrapper"}
                <ProjectListComponent/>
            </div>
        }
    }
}
