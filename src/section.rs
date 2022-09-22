use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    pub class: Classes,
}

pub struct Section;

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div class={classes!("py-3", ctx.props().class.clone())}>
                <div class="max-w-6xl mx-auto">
                    {for ctx.props().children.iter()}
                </div>
            </div>
        };
    }
}
