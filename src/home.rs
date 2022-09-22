use crate::navbar::{Navbar, NavbarLink};
use crate::section::Section;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
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
