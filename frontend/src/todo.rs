use crate::models::todo::Todo;
use crate::todo_list::{TodoID, TodoListComp, Msg as ParentMsg};
use log::{debug,info};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_styles::layouts::container::{Container,Wrap,Direction};
use yew_styles::layouts::item::{Item,ItemLayout};

pub struct TodoComp {
    link: ComponentLink<Self>,
    props: Props,
    open: bool,
}

#[derive(Properties, Clone,PartialEq,Eq)]
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        props == self.props
    }

    fn view(&self) -> Html {
        info!("rendering: {:?}",self.props.data);
        let toggle = self.link.callback(|_| Msg::ToggleOpen);
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(50)) >
                    <b onclick=toggle> {&self.props.data.title} </b>
                    {
                        if let Some(details) = &self.props.data.details{
                            html!{<>{" - "}{details}</>}
                        }else {
                            html!{}
                        }
                    }
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(4)) >
                {self.list_data()}
                </Item>
            </Container>
        }
    }
}

impl TodoComp {
    pub fn list_data(&self) -> Html {
        if self.open {
            html!{
                <TodoListComp id=TodoID::Parent(self.props.data.id)/>
            }
        }else {
            html!{}
        }
    }
}
