use crate::models::todo::Todo;
use crate::todo_list::{TodoID, TodoListComp};
use log::debug;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct TodoComp {
    link: ComponentLink<Self>,
    props: Props,
    open: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub data: Todo,
    #[prop_or(false)]
    pub open: bool,
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
            open: props.open,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            ToggleOpen => {
                self.open = !self.open;
                true
            }
            NoOp => {
                debug!("here");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let toggle = self.link.callback(|_| Msg::ToggleOpen);
        let complete = self.link.callback(|_| Msg::NoOp);
        html! {
            <div >
                <b ondblclick=complete onclick=toggle> {&self.props.data.title} </b>
                {
                    if let Some(details) = &self.props.data.details{
                        html!{<>{" - "}{details}</>}
                    }else {
                        html!{}
                    }
                }
                {
                    if self.open {
                        html!{
                            <TodoListComp id=TodoID::Parent(self.props.data.id)/>
                        }
                    }else {
                        html!{}
                    }
                }
            </div>
        }
    }
}
