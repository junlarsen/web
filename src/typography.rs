use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ParagraphProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Paragraph)]
pub fn paragraph(props: &ParagraphProps) -> Html {
    return html! {
        <p class="font-noto text-gray-12 mt-2">
            {for props.children.iter()}
        </p>
    };
}

#[derive(Properties, PartialEq)]
pub struct HeadingProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub primary: bool,
}

#[function_component(Heading)]
pub fn heading(props: &HeadingProps) -> Html {
    return html! {
        <@{if props.primary { "h1" } else { "h2" }} class={classes!(
            "font-bold", "underline", "decoration-secondary-10", "decoration-4", "font-noto", "tracking-tighter", "text-primary-12",
            if props.primary { "text-4xl" } else { "text-3xl" }
        )}>
            {for props.children.iter()}
        </@>
    };
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub children: Children,
    pub href: String,
    #[prop_or(false)]
    pub external: bool,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    return html! {
        <a
            class="text-primary-9 hover:text-primary-10"
            target="_blank"
            rel="noopener noreferrer"
            href={props.href.clone()}
        >
            {for props.children.iter()}
        </a>
    };
}
