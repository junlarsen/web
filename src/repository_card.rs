use crate::icon::Icon;
use crate::typography::{Link, Paragraph};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepositoryCardProps {
    pub org: String,
    pub repo: String,
    pub description: String,
    pub contribution: String,
    #[prop_or(vec![])]
    pub stack: Vec<String>,
}

#[function_component(RepositoryCard)]
pub fn repository_card(props: &RepositoryCardProps) -> Html {
    return html! {
        <div class="border rounded border-gray-4 shadow p-2">
            <div class="flex items-center gap-y-1 gap-x-2 flex-row">
                <img class="rounded-full" src={format!("https://github.com/{}.png?size=48", props.org)} alt={format!("{} organization icon", props.org)} width={24} height={24} />
                <span class="font-bold">{format!("{}/{}", props.org, props.repo)}</span>
            </div>

            <div class="flex items-start gap-y-1 gap-x-2 flex-row mt-2">
                <div class="flex flex-col gap-1 min-w-[24px] mt-2">
                    {for props.stack.iter().map(|name| html! {
                        <Icon name={name.to_owned()} alt={name.to_owned()} />
                    })}
                </div>
                <div class="flex flex-col gap-3">
                    <Paragraph>{props.description.clone()}</Paragraph>
                    <Paragraph>{props.contribution.clone()}</Paragraph>
                    <div>
                        <Link external={true} href={format!("https://github.com/{}/{}", props.org, props.repo)}>
                            {"Visit repo"}
                        </Link>
                    </div>
                </div>
            </div>
        </div>
    };
}
