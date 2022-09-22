use crate::navbar::Navbar;
use crate::navbar_link::NavbarLink;
use crate::paragraph::Paragraph;
use crate::section::Section;
use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <main class="min-h-screen bg-gray-2">
                <Section class="bg-gray-2">
                    <Navbar>
                        <NavbarLink href="#">{"About"}</NavbarLink>
                        <NavbarLink href="#">{"Projects"}</NavbarLink>
                        <NavbarLink href="#">{"Experience"}</NavbarLink>
                        <NavbarLink href="#">{"Resume"}</NavbarLink>
                    </Navbar>
                </Section>
            </main>
        };
    }
}
