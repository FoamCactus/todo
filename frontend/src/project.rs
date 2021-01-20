use crate::models::project::Project;
use crate::models::todo::TodoID;
use crate::todo_list::TodoListComp;
use yew::MouseEvent;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Props {
    pub data: Project,
    #[prop_or(false)]
    pub open: bool,
}

pub enum Msg {
    ToggleOpen,
}

pub struct ProjectComp {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ProjectComp {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            ToggleOpen => {
                self.props.open = !self.props.open;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let open_click = self.link.callback(|_: MouseEvent| Msg::ToggleOpen);

        html! {
            <div>
                <h2 onclick=open_click>
                    {&self.props.data.title}
                </h2>
                 {
                     if self.props.open {
                         html!{
                             <>
                                 <TodoListComp id=TodoID::Project(self.props.data.id)/>
                             </>
                         }


                     }else {
                         html!{}
                     }
                 }
            </div>
        }
    }
}
