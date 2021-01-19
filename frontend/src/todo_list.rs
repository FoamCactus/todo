use crate::error::ServiceError;
use crate::models::todo::{Todo, TodoBuilder};
use crate::service::{Service, TodoService};
use crate::todo::TodoComp;
use log::{info,error};
use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::{ChangeData, MouseEvent};
use yew_styles::button::Button;
use yew_styles::styles::{Palette,Style,Size};
use yew_styles::forms::form_input::{FormInput, InputType};

#[derive(Clone, Copy)]
pub enum TodoID {
    Parent(i32),
    Project(i32),
}

pub struct TodoListComp {
    link: ComponentLink<Self>,
    props: Props,
    service: TodoService,
    todos: Vec<Todo>,
    task: Option<FetchTask>,
    title: Option<String>,
    details: Option<String>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: TodoID,
}

pub enum Msg {
    NoOp,
    Loaded(Result<Vec<Todo>, ServiceError>),
    SetTitle(ChangeData),
    SetDetails(ChangeData),
    SaveNew,
    Push(Todo),
    Remove(i32),
    Complete(usize)
}

impl Component for TodoListComp {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            service: TodoService::new(),
            todos: Vec::new(),
            task: None,
            title: None,
            details: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.link.callback(|r| Msg::Loaded(r));
            self.task = Some(match self.props.id {
                TodoID::Parent(id) => self.service.get_by_parent(id, callback),
                TodoID::Project(id) => self.service.get_by_project(id, callback),
            });
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Loaded(Ok(vec)) => {
                self.todos = vec;
                self.task = None;
                true
            }
            SetTitle(ChangeData::Value(val)) => {
                self.title = Some(val);
                false
            }
            SetDetails(ChangeData::Value(val)) => {
                self.details = Some(val);
                false
            }
            SaveNew => {
                if let Some(title) = &self.title {
                    let callback = self.link.callback(|t| match t {
                        Ok(Some(t)) => Msg::Push(t),
                        _ => Msg::NoOp,
                    });
                    let mut builder = TodoBuilder::new(&title);
                    if let Some(details) = &self.details {
                        builder.with_details(&details);
                    }
                    match self.props.id {
                        TodoID::Parent(id) => {
                            self.task =
                                Some(self.service.save(builder.parent(id).build(), callback))
                        }
                        TodoID::Project(id) => {
                            self.task =
                                Some(self.service.save(builder.project(id).build(), callback))
                        }
                    }
                }
                false
            }
            Push(t) => {
                info!("{:?}",t);
                self.todos.push(t);
                true
            },
            Complete(t) => {
                info!("index removed:{}",t);
                let mut complete_todo = self.todos.remove(t);
                complete_todo.complete = true;
                info!("todo:{:?}",complete_todo);
                let callback = self.link.callback(|_| Msg::NoOp);
                self.task = Some( self.service.mark_complete(complete_todo,callback));
                true
            },
            NoOp => {
                info!("here");
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {self.input_html()}
                <ul>
                {
                    self.list_todos()
                }
                </ul>
            </div>
        }
    }
}

impl TodoListComp {
    fn input_html(&self) -> Html {
        let save_click = self.link.callback(|_: MouseEvent| Msg::SaveNew);
        let update_todo_title = self.link.callback(|data: ChangeData| Msg::SetTitle(data));
        let update_todo_details = self.link.callback(|data: ChangeData| Msg::SetDetails(data));
        html! {
            <>
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
            </>
        }
    }

    fn list_todos(&self) -> Html {
        self.todos.iter().enumerate().map(|t| self.single_todo(t)).collect::<Html>()
    }

    fn single_todo(&self, (place,todo):(usize,&Todo)) -> Html {
        let comp = self.link.callback(move |_|{ Msg::Complete(place)});
        html! {
            <li key=todo.id>
                <div>
                    <Button
                        onclick_signal=comp
                        button_palette=Palette::Primary
                        button_style=Style::Light
                        button_size=Size::Small
                    >
                        {"Complete"}
                    </Button>
                    <TodoComp data=todo/>
                </div>
            </li>
        }
    }

}
