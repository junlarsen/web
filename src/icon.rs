use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub name: String,
    #[prop_or_default]
    pub alt: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    return html! {
        <img src={format!("/{}.svg", props.name)} alt={props.alt.to_owned()} width={24} />
    };
}
