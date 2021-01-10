use yew::{ShouldRender,Html,html,ComponentLink,Component,Properties,Callback};
use crate::service::{Service,TodoService};
use crate::models::project::Project;

#[derive(Properties,Clone)]
pub struct Props{
    pub data: Project,
    #[prop_or(false)]
    pub open: bool
}

pub enum Msg {
    ToggleOpen
}

pub struct ProjectComp {
    link: ComponentLink<Self>,
    props: Props
}

impl Component for ProjectComp {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            props
        }
    }

    fn update(&mut self, msg:Self::Message) -> ShouldRender {
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
        html!{
            <div>
                {&self.props.data.title}
            </div>
        }
    }

}


