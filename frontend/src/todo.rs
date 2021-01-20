use crate::models::todo::Todo;
use crate::todo_list::{TodoID, TodoListComp, Msg as ParentMsg};
use log::{debug,info};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_styles::layouts::container::{Container,Wrap,Direction};
use yew_styles::layouts::item::{Item,ItemLayout};
use yew::Callback;
use yew::MouseEvent;

pub struct TodoComp {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub data: Todo,
    pub on_click_signal: Callback<MouseEvent>,
}

pub enum Msg {
    ToggleOpen,
    NoOp,
}

impl Component for TodoComp {
    type Properties = Props;
    type Message = Msg;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }


    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(50)) >
                    <b onclick=self.props.on_click_signal.clone()> {&self.props.data.title} </b>
                    {
                        if let Some(details) = &self.props.data.details{
                            html!{<>{" - "}{details}</>}
                        }else {
                            html!{}
                        }
                    }
                </Item>
            </Container>
        }
    }
}

