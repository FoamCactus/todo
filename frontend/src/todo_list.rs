use crate::error::ServiceError;
use crate::models::todo::{Todo, TodoBuilder,TodoID};
use crate::service::{Service, TodoService};
use crate::todo::TodoComp;
use wasm_bindgen::JsCast;
use log::info;
use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::{ChangeData, MouseEvent};
use yew::utils;
use yew::web_sys::HtmlInputElement;
use yew_styles::button::Button;
use yew_styles::styles::{Palette,Style,Size};
use yew_styles::forms::form_input::{FormInput, InputType};
use yew_styles::forms::form_label::FormLabel;
use yew_styles::forms::form_group::{FormGroup,Orientation};
use yew_styles::layouts::container::{Container,Wrap,Direction,AlignContent};
use yew_styles::layouts::container::Mode::SafeMode;
use yew_styles::layouts::item::Item;
use yew_styles::layouts::item::ItemLayout::{ItXs,ItS,ItM,ItL,ItXl};


pub struct TodoListComp {
    link: ComponentLink<Self>,
    props: Props,
    service: TodoService,
    todos: Vec<Todo>,
    task: Option<FetchTask>,
    title: Option<String>,
    details: Option<String>,
}

#[derive(Properties, Clone,Debug)]
pub struct Props {
    pub id: TodoID,
    #[prop_or(false)]
    pub open: bool,
    #[prop_or(None)]
    pub parent_todo: Option<Todo>
}

pub enum Msg {
    NoOp,
    Loaded(Result<Vec<Todo>, ServiceError>),
    SetTitle(ChangeData),
    SetDetails(ChangeData),
    SaveNew,
    Push(Todo),
    Complete(usize),
    ToggleOpen
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
            info!("{:?}",self.props);
            self.task = Some(match self.props.id {
                TodoID::Parent(id) => self.service.get_by_parent(id, callback),
                TodoID::Project(id) => self.service.get_by_project(id, callback),
            });
            if let TodoID::Project(_) = self.props.id {
                self.props.open = true;
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Loaded(Ok(vec)) => {
                info!("{:?}",vec);
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
                    builder.id(self.props.id);
                    self.task = Some(self.service.save(builder,callback));
                }
                false
            }
            Push(t) => {
                info!("{:?}",t);
                self.remove_input_values();
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
            },
            ToggleOpen => {
                self.props.open = !self.props.open;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let open_callback = self.link.callback(|_:MouseEvent| Msg::ToggleOpen);
        html! {
            <div>
                {
                    if let Some(d) = &self.props.parent_todo {
                        html!{
                            <TodoComp data=d on_click_signal=open_callback/>
                        }

                    }else{
                        html!{}
                    }
                }
                {
                    if self.props.open {
                        html!{
                            <>
                            {self.input_html()}
                            {self.list_todos()}
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

impl TodoListComp {
    fn input_html(&self) -> Html {
        let save_click = self.link.callback(|_: MouseEvent| Msg::SaveNew);
        let update_todo_title = self.link.callback(|data: ChangeData| Msg::SetTitle(data));
        let update_todo_details = self.link.callback(|data: ChangeData| Msg::SetDetails(data));
        let button_layout = vec!(ItXs(1),ItS(1),ItM(1),ItL(1),ItXl(2));
        let title_layout = vec!(ItXs(4),ItS(4),ItM(4),ItL(4),ItXl(4));
        let details_layout = vec!(ItXs(5),ItS(5),ItM(5),ItL(5),ItXl(4));
        let input_ids = self.get_input_ids();
        let title_id = input_ids.get(0).unwrap();
        let detail_id = input_ids.get(1).unwrap();
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap>
                <Item layouts=title_layout>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="Title: " label_for=title_id.clone()/>
                        <FormInput
                            id=title_id.clone()
                            input_type=InputType::Text
                            required=true
                            underline=true
                            input_size=Size::Small
                            name="title"
                            onchange_signal=update_todo_title
                        />
                    </FormGroup>
                </Item>
                <Item layouts=details_layout>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="Details: " label_for=detail_id.clone()/>
                        <FormInput
                            id=detail_id.clone()
                            input_type=InputType::Text
                            underline=true
                            required=false
                            input_size=Size::Small
                            name="details"
                            onchange_signal=update_todo_details
                        />
                    </FormGroup>
                </Item>
                <Item layouts=button_layout>
                    <Button
                        onclick_signal=save_click
                    >
                    {"Save"}
                    </Button>
                </Item>
            </Container>
        }
    }

    fn list_todos(&self) -> Html {
        if self.props.open {
            html!{
                <ul>
                    {
                    self.todos.iter().enumerate().map(|t| self.single_todo(t)).collect::<Html>()
                    }
                </ul>
            }
        }else {
            html!{}
        }
    }

    fn single_todo(&self, (place,todo):(usize,&Todo)) -> Html {
        let comp = self.link.callback(move |_|{ Msg::Complete(place)});
        let id = TodoID::Parent(todo.id);
        let button_layout = vec!(ItXs(1),ItS(1),ItM(1),ItL(1),ItXl(1));
        let data_layout = vec!(ItXs(11),ItS(11),ItM(11),ItL(11),ItXl(11));
        html! {
            <li key=todo.id>
                <Container direction=Direction::Row wrap=Wrap::Wrap align_content=AlignContent::Start(SafeMode)>
                    <Item layouts=button_layout>
                        <Button
                            onclick_signal=comp
                            button_palette=Palette::Primary
                            button_style=Style::Light
                            button_size=Size::Small
                        >
                            {"Complete"}
                        </Button>
                    </Item>
                    <Item layouts=data_layout>
                        <TodoListComp parent_todo=todo id=id/>
                    </Item>

                </Container>
            </li>
        }
    }

    fn get_input_ids(&self) -> Vec<String> {
        let id_text = match self.props.id {
            TodoID::Parent(id) => format!("parent_{}",id),
            TodoID::Project(id) => format!("project_{}",id),
        };
        vec![format!("title_{}",&id_text),format!("details_{}",&id_text)]
    }

    fn remove_input_values(&self) {
        let input_ids = self.get_input_ids();

        for id in input_ids {
            let input_from_element = utils::document()
                .get_element_by_id(&id)
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();

            input_from_element.set_value("");
        }
    }
}
