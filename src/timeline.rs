use crate::icon::Icon;
use crate::typography::Paragraph;
use js_sys::Date;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct TimelineEvent {
    pub start: Date,
    pub end: Date,
    pub title: String,
    pub roles: Vec<String>,
    pub stack: Vec<String>,
    pub active: bool,
}

impl ImplicitClone for TimelineEvent {}

impl TimelineEvent {
    pub fn new(
        start: Date,
        end: Date,
        title: String,
        roles: Vec<String>,
        stack: Vec<String>,
    ) -> Self {
        Self {
            start,
            end,
            title,
            roles,
            stack,
            active: false,
        }
    }

    pub fn active(mut self) -> TimelineEvent {
        self.active = true;
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    #[prop_or(vec![])]
    pub events: Vec<(String, TimelineEvent)>,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    return html! {
        <div class="flex flex-col gap-2">
            {for props.events.iter().map(|(title, event)| html! {
                <div class="w-full flex flex-row border-l-4 border-secondary-10">
                    <div class="w-1/6 pl-4 font-bold flex items-center">
                        <Paragraph>{title.clone()}</Paragraph>
                    </div>
                    <div class="w-5/6">
                        <TimelineItem event={event.clone()} />
                    </div>
                </div>
            })}
        </div>
    };
}

#[derive(Properties, PartialEq)]
pub struct TimelineItemProps {
    pub event: TimelineEvent,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &TimelineItemProps) -> Html {
    return html! {
        <div class="border rounded border-gray-4 shadow p-2">
            <div class="flex flex-row justify-between">
                <span class="font-bold">{props.event.title.clone()}</span>
                <span class="font-bold">{format_date_range(&props.event.start, &props.event.end, props.event.active)}</span>
            </div>

            <div class="flex items-start gap-y-1 gap-x-2 flex-row my-2">
                <div class="flex flex-col gap-3">
                    {for props.event.roles.iter().map(|role| html! {
                        <Paragraph>{role.clone()}</Paragraph>
                    })}
                </div>
            </div>

            <div class="flex flex-row gap-1">
                {for props.event.stack.iter().map(|technology| html! {
                    <Icon name={technology.clone()} alt={technology.clone()} />
                })}
            </div>
        </div>
    };
}

fn format_date_range(start: &Date, end: &Date, active: bool) -> String {
    const MONTHS: [&str; 12] = [
        "Jan.", "Feb.", "Mar.", "Apr.", "May", "Jun.", "Jul.", "Aug.", "Oct.", "Sep.", "Nov.",
        "Dec.",
    ];

    let start_year = start.get_full_year();
    let start_month = MONTHS[(start.get_month() - 1) as usize];
    let end_year = end.get_full_year();
    let end_month = MONTHS[(end.get_month() - 1) as usize];
    if active {
        format!("{} {} — Present", start_month, start_year)
    } else {
        format!(
            "{} {} — {} {}",
            start_month, start_year, end_month, end_year
        )
    }
}
