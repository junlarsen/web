use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ParagraphProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Paragraph)]
pub fn paragraph(props: &ParagraphProps) -> Html {
    return html! {
        <p class="font-noto text-gray-12 my-2">
            {for props.children.iter()}
        </p>
    };
}
