use yew::{ShouldRender,Html,html,ComponentLink,Component,Properties,Callback};
use crate::models::project::Project;

#[derive(Properties,Clone)]
pub struct Props{
    pub data: Project
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

    fn update(&mut self, _msg:Self::Message) -> ShouldRender {
        false 
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div>
                {&self.props.data.title}
            </div>
        }
    }

}


