use yew::{ShouldRender,Html,html,ComponentLink,Component,Properties,Callback};
use crate::models::todo::Todo;

pub struct TodoComp {
    link: ComponentLink<Self>,
    props: Props
}

#[derive(Properties,Clone)]
pub struct Props {
    pub data: Todo
}

impl Component for TodoComp {
    type Properties = Props;
    type Message = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
       false 
    }

    fn view(&self) -> Html {
        html!{
            <div>
                <b> {&self.props.data.title} </b>
                {
                    if let Some(details) = &self.props.data.details{
                        html!{<>{" - "}{details}</>}
                    }else {
                        html!{}
                    }
                }
            </div>
        }
    }





}
