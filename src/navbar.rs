use crate::navbar_link::NavbarLink;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = NavbarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <nav class="flex w-full flex-col md:flex-row md:justify-between p-2">
                <div class="flex w-full justify-between">
                    <NavbarLink href="#">{"٩(◕‿◕｡)۶"}</NavbarLink>
                    <div class="hidden md:flex flex-col md:flex-row gap-x-4">
                        {for ctx.props().children.iter()}
                    </div>
                </div>
            </nav>
        };
    }
}
