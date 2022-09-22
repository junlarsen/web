use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarLinkProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or("#")]
    pub href: &'static str,
}

pub struct NavbarLink;

impl Component for NavbarLink {
    type Message = ();
    type Properties = NavbarLinkProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <a href={ctx.props().href} class="align-middle text-xl p-1 font-bold text-primary-12 decoration-4 transition ease-in-out decoration-primary hover:underline">
                {for ctx.props().children.iter()}
            </a>
        };
    }
}
