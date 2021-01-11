use crate::error::ServiceError;
use crate::models::project::Project;
use crate::models::todo::{Todo, TodoBuilder};
use crate::service::{Service, TodoService};
use crate::todo::TodoComp;
use crate::todo_list::{TodoListComp,TodoID};
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::{ChangeData, MouseEvent};
use yew_styles::button::Button;
use yew_styles::forms::form_input::{FormInput, InputType};

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
    service: TodoService,
    task: Option<FetchTask>,
    todos: Vec<Todo>,
    title: Option<String>,
    details: Option<String>,
    props: Props,
}

impl Component for ProjectComp {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            service: TodoService::new(),
            todos: Vec::new(),
            task: None,
            title: None,
            details: None,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            ToggleOpen => {
                self.props.open = !self.props.open;
                true
            }
            _ => false,
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
                <p onclick=open_click>
                    {&self.props.data.title}
                </p>
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
