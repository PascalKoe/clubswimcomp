use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

mod competition;
mod event;
mod page;
mod participant;

pub use competition::*;
pub use event::*;
pub use page::*;
pub use participant::*;

#[component]
pub fn GenderDisplay(
    #[prop(into)] gender: MaybeSignal<model::Gender>,
    #[prop(optional)] short: bool,
) -> impl IntoView {
    move || match (gender(), short) {
        (model::Gender::Female, false) => "Female",
        (model::Gender::Female, true) => "F",
        (model::Gender::Male, false) => "Male",
        (model::Gender::Male, true) => "M",
    }
}

#[component]
pub fn DistanceDisplay(#[prop(into)] distance: MaybeSignal<u32>) -> impl IntoView {
    view! {
        {distance} "m"
    }
}

#[component]
pub fn StrokeDisplay(#[prop(into)] stroke: MaybeSignal<model::Stroke>) -> impl IntoView {
    move || match stroke() {
        model::Stroke::Butterfly => "Butterfly",
        model::Stroke::Back => "Backstroke",
        model::Stroke::Breast => "Breaststroke",
        model::Stroke::Freestyle => "Freestyle",
    }
}

#[component]
pub fn BirthdayDisplay(#[prop(into)] birthday: MaybeSignal<chrono::NaiveDate>) -> impl IntoView {
    move || birthday().format("%Y-%m-%d").to_string()
}

#[component]
pub fn TimeDisplay(#[prop(into)] millis: MaybeSignal<i64>) -> impl IntoView {
    let minutes = move || millis() / (60 * 1000);
    let seconds = move || (millis() / 1000) % 60;
    let hundreths = move || (millis() / 10) % 100;

    move || format!("{:02}:{:02},{:02}", minutes(), seconds(), hundreths())
}

#[component]
pub fn FormItem(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <label class="form-control w-full max-w-2xl">
            <div class="label">
                <span class="label-text">{label}</span>
            </div>
            {children()}
        </label>
    }
}

#[component]
pub fn ActionRow(children: Children) -> impl IntoView {
    view! {
        <div class="mb-8">
            { children() }
        </div>
    }
}

#[component]
pub fn CellGender(#[prop(into)] gender: MaybeSignal<model::Gender>) -> impl IntoView {
    view! {
        <td>
            <GenderDisplay gender />
        </td>
    }
}

#[component]
pub fn CellDistance(#[prop(into)] distance: MaybeSignal<u32>) -> impl IntoView {
    view! {
        <td>
            <DistanceDisplay distance />
        </td>
    }
}

#[component]
pub fn CellStroke(#[prop(into)] stroke: MaybeSignal<model::Stroke>) -> impl IntoView {
    view! {
        <td>
            <StrokeDisplay stroke />
        </td>
    }
}

#[component]
pub fn CellTime(#[prop(into)] millis: MaybeSignal<i64>) -> impl IntoView {
    view! {
        <td>
            <TimeDisplay millis />
        </td>
    }
}

#[component]
pub fn CellDisqualified(#[prop(into)] disqualified: bool) -> impl IntoView {
    view! {
        <td>
            {
                match disqualified {
                    true => "Disqualified",
                    false => "-"
                }
            }
        </td>
    }
}

#[component]
pub fn HeadingsCompetition() -> impl IntoView {
    view! {
        <th>Distance</th>
        <th>Stroke</th>
        <th>Gender</th>
    }
}

#[component]
pub fn CellsCompetition(
    #[prop(into)] competition: MaybeSignal<model::Competition>,
) -> impl IntoView {
    let c = competition.clone();
    let distance = MaybeSignal::derive(move || c().distance);

    let c = competition.clone();
    let stroke = MaybeSignal::derive(move || c().stroke);

    let c = competition.clone();
    let gender = MaybeSignal::derive(move || c().gender);

    view! {
        <CellDistance distance />
        <CellStroke stroke />
        <CellGender gender />
    }
}

#[component]
pub fn HeadingsResult() -> impl IntoView {
    view! {
        <th>Disqualified</th>
        <th>Time</th>
    }
}

#[component]
pub fn CellsResult(result: Option<model::RegistrationResult>) -> impl IntoView {
    let Some(result) = result else {
        return view! {
            <td></td>
            <td></td>
        }
        .into_view();
    };

    view! {
        <CellDisqualified disqualified=result.disqualified />
        <CellTime millis=result.time_millis />
    }
    .into_view()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ActionType {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Error,
}

impl ActionType {
    pub fn as_class(&self) -> &'static str {
        match self {
            ActionType::Default => "",
            ActionType::Neutral => "btn-neutral",
            ActionType::Primary => "btn-primary",
            ActionType::Secondary => "btn-secondary",
            ActionType::Error => "btn-error",
        }
    }
}

#[component]
pub fn TableIconButton(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
) -> impl IntoView {
    let btn_class = action_type.as_class();
    view! {
        <button class=format!("btn btn-xs {btn_class}")>
            {children()}
        </button>
    }
}

#[component]
pub fn CellIconButton(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
) -> impl IntoView {
    view! {
        <td class="w-0">
            <TableIconButton action_type>
                {children()}
            </TableIconButton>
        </td>
    }
}

#[component]
pub fn TableIconLink(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
    #[prop(into)] href: MaybeSignal<String>,
) -> impl IntoView {
    let btn_class = action_type.as_class();
    view! {
        <A class=format!("btn btn-xs {btn_class}") href>
            {children()}
        </A>
    }
}

#[component]
pub fn CellIconLink(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
    #[prop(into)] href: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <td class="w-0">
            <TableIconLink action_type href>
                {children()}
            </TableIconLink>
        </td>
    }
}

#[component]
pub fn ActionButton(children: Children, action_type: ActionType) -> impl IntoView {
    let btn_class = action_type.as_class();
    view! {
        <button class=format!("btn btn-sm {btn_class} rounded-full mr-4")>
            {children()}
        </button>
    }
}
