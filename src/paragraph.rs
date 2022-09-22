use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ParagraphProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Paragraph;

impl Component for Paragraph {
    type Message = ();
    type Properties = ParagraphProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <p class="font-noto text-gray-12 my-2">
                {for ctx.props().children.iter()}
            </p>
        };
    }
}
