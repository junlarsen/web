use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    pub class: Classes,
    pub id: String,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    return html! {
        <div class={classes!("p-3", props.class.clone())} id={props.id.clone()}>
            <div class="max-w-4xl mx-auto">
                {for props.children.iter()}
            </div>
        </div>
    };
}
