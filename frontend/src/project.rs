use yew::{ShouldRender,Html,html,ComponentLink,Component,Properties,Callback};
use yew::{MouseEvent,ChangeData};
use crate::todo::TodoComp;
use crate::service::{Service,TodoService};
use crate::error::ServiceError;
use crate::models::project::Project;
use crate::models::todo::{Todo,TodoBuilder};
use yew::services::fetch::FetchTask;
use yew_styles::button::Button;
use yew_styles::forms::form_input::{FormInput,InputType};

#[derive(Properties,Clone)]
pub struct Props{
    pub data: Project,
    #[prop_or(false)]
    pub open: bool
}

pub enum Msg {
    ToggleOpen,
    Loaded(Result<Vec<Todo>,ServiceError>),
    SetTitle(ChangeData),
    SetDetails(ChangeData),
    SaveNew,
    Push(Todo),
    NoOp
}

pub struct ProjectComp {
    link: ComponentLink<Self>,
    service: TodoService,
    task: Option<FetchTask>,
    todos: Vec<Todo>,
    title: Option<String>,
    details: Option<String>,
    props: Props
}

impl Component for ProjectComp {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            service: TodoService::new(),
            todos: Vec::new(),
            task: None,
            title: None,
            details: None,
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            ToggleOpen => {
                self.props.open = !self.props.open;
                true
            },
            Loaded(Ok(vec)) => {
                self.todos = vec;
                self.task = None;
                true
            },
            SetTitle(ChangeData::Value(val)) => {
                self.title = Some(val);
                false
            },
            SetDetails(ChangeData::Value(val)) => {
                self.details = Some(val);
                false
            },
            SaveNew => {
                if let Some(title) = &self.title{
                    let callback = self.link.callback(|t|
                                                      match t {
                                                          Ok(Some(t)) => Msg::Push(t),
                                                          _ => Msg::NoOp
                                                      }
                                                      );
                    let mut builder = TodoBuilder::new(self.props.data.id,&title);
                    if let Some(details) = &self.details {
                        builder.with_details(&details);
                    }
                    self.task = Some(self.service.save(builder.build(), callback));
                }
                false
            },
            Push(t) => {
                self.todos.push(t);
                true
            }
            _ => false
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.link.callback(|r| Msg::Loaded(r));
            self.task = Some(self.service.get_by_project(self.props.data.id,callback))
        }
    }

    

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let open_click = self.link.callback(|_:MouseEvent| Msg::ToggleOpen);
        let save_click = self.link.callback(|_:MouseEvent| Msg::SaveNew);
        let update_todo_title = self.link.callback(|data:ChangeData| Msg::SetTitle(data));
        let update_todo_details = self.link.callback(|data:ChangeData| Msg::SetDetails(data));

        html!{
            <div ondblclick=open_click>
                {&self.props.data.title}
                 {
                     if self.props.open {
                         html!{
                             <div>
                                 <FormInput 
                                     input_type=InputType::Text
                                     required=true
                                     name="title"
                                     onchange_signal=update_todo_title
                                 />
                                 <FormInput 
                                     input_type=InputType::Text
                                     required=false
                                     name="details"
                                     onchange_signal=update_todo_details
                                 />
                                 <Button
                                     onclick_signal=save_click
                                 >
                                 {"Save"}
                                 </Button>
                                 <ul>
                                 {
                                     self.todos.iter().rev().map(|t|{
                                         html!{
                                             <li>
                                                 <TodoComp data=t/>
                                             </li>
                                         }

                                     }).collect::<Html>()
                                 }
                                 </ul>
                             </div>
                         }
                     }else {
                         html!{}
                     }
                 }
            </div>
        }
    }

}


