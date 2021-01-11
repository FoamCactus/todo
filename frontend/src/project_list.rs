use crate::error::ServiceError;
use crate::models::project;
use crate::project::ProjectComp;
use crate::service::{ProjectService, Service};
use log::info;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::{ChangeData, MouseEvent};
use yew_styles::button::Button;
use yew_styles::forms::form_input::{FormInput, InputType};

pub struct ProjectListComponent {
    link: ComponentLink<Self>,
    service: ProjectService,
    projects: Vec<project::Project>,
    task: Option<FetchTask>,
    new_project: project::NewProject,
}

pub enum Msg {
    Loaded(Result<Vec<project::Project>, ServiceError>),
    ChangeTitle(ChangeData),
    PostButton,
    Saved(Result<Option<project::Project>, ServiceError>),
}

impl Component for ProjectListComponent {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            service: ProjectService::new(),
            projects: Vec::new(),
            task: None,
            new_project: project::NewProject::new(),
            link,
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.link.callback(|v| Self::Message::Loaded(v));
            self.task = Some(self.service.all(callback));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Loaded(Ok(v)) => {
                info!("loaded");
                self.projects = v;
                true
            }
            ChangeTitle(data) => {
                if let ChangeData::Value(v) = data {
                    info!("setting title to: {}", v);
                    self.new_project.set_title(&v);
                }
                false
            }
            PostButton => {
                let callback = self.link.callback(|v| Msg::Saved(v));
                self.task = Some(self.service.save(self.new_project.clone(), callback));
                self.new_project = project::NewProject::new();
                false
            }
            Saved(Ok(Some(proj))) => {
                info!("saved and returned");
                self.projects.push(proj);
                true
            }
            Saved(Err(e)) => {
                info!("{:?}", e);
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_change_title = self
            .link
            .callback(|change: ChangeData| Msg::ChangeTitle(change));
        let save_button = self.link.callback(|_: MouseEvent| Msg::PostButton);
        html! {
            <div>
                <label for="newTitle">{"Title"}</label>
                <FormInput
                    input_type=InputType::Text
                    required=true
                    name="newTitle"
                    id="newTitle"
                    onchange_signal=on_change_title
                />
                <Button
                    onclick_signal=save_button
                >
                    {"Save"}
                </Button>
                {
                    self.projects.iter().map(
                        |p| html!{<ProjectComp data=p></ProjectComp>}
                    ).collect::<Html>()
                }
            </div>
        }
    }
}
