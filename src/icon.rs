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
        <img title={props.alt.clone()} src={format!("/{}.svg", props.name)} alt={props.alt.clone()} width={24} height={24} />
    };
}
