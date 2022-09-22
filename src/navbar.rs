use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    return html! {
        <nav class="flex w-full flex-col md:flex-row md:justify-between px-3">
            <div class="flex w-full justify-between">
                <NavbarLink href={"#top".to_owned()}>{"٩(◕‿◕｡)۶"}</NavbarLink>
                <div class="hidden md:flex flex-col md:flex-row gap-x-4">
                    {for props.children.iter()}
                </div>
            </div>
        </nav>
    };
}

#[derive(Properties, PartialEq)]
pub struct NavbarLinkProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or("#".to_owned())]
    pub href: String,
}

#[function_component(NavbarLink)]
pub fn navbar_link(props: &NavbarLinkProps) -> Html {
    return html! {
        <a href={props.href.clone()} class="align-middle text-xl py-1 font-bold text-primary-12 decoration-4 transition ease-in-out decoration-secondary-10 hover:underline">
            {for props.children.iter()}
        </a>
    };
}
