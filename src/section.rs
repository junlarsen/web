use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    pub class: Classes,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    return html! {
        <div class={classes!("py-3", props.class.clone())}>
            <div class="max-w-6xl mx-auto">
                {for props.children.iter()}
            </div>
        </div>
    };
}
