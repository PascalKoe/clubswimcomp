use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

mod competition;
mod event;
mod group;
mod page;
mod participant;
mod registrations;
mod scanner;

pub use competition::*;
pub use event::*;
pub use group::*;
pub use page::*;
pub use participant::*;
pub use registrations::*;
pub use scanner::*;
use uuid::Uuid;

use crate::api_client;

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
pub fn CellTime(millis: i64) -> impl IntoView {
    view! {
        <td>
            <TimeDisplay millis />
        </td>
    }
}

#[component]
pub fn CellDisqualified(disqualified: bool) -> impl IntoView {
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
        <th>Target Time</th>
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

    let c = competition.clone();

    view! {
        <CellDistance distance />
        <CellStroke stroke />
        <CellGender gender />
        <CellTime millis=c().target_time />
    }
}

#[component]
pub fn HeadingsResult() -> impl IntoView {
    view! {
        <th>Disqualified</th>
        <th>Time</th>
        <th>Points</th>
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
        <td>
            {result.fina_points}
        </td>
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

#[component]
pub fn InputTime(#[prop(into)] set_time: WriteSignal<Option<u32>>) -> impl IntoView {
    let input_changed = move |ev| {
        let value = event_target_value(&ev);
        if value.len() != 6 {
            set_time(None);
            return;
        }

        let Ok(min) = value[0..=1].parse::<u32>() else {
            set_time(None);
            return;
        };

        let sec = match value[2..=3].parse::<u32>() {
            Ok(sec) if sec < 60 => sec,
            _ => {
                set_time(None);
                return;
            }
        };

        let Ok(hundredths) = value[4..=5].parse::<u32>() else {
            set_time(None);
            return;
        };

        let millis = (min * 60 * 1000) + (sec * 1000) + (hundredths * 10);
        set_time(Some(millis));
    };

    view! {
        <input
            class="input input-bordered"
            type="text"
            minlength=6
            maxlength=6
            on:input=input_changed
        />
    }
}

#[component]
pub fn InputGender(#[prop(into)] set_gender: WriteSignal<model::Gender>) -> impl IntoView {
    let input_changed = move |ev| {
        let value = event_target_value(&ev);
        let g = match value.as_str() {
            "F" => model::Gender::Female,
            "M" => model::Gender::Male,
            _ => model::Gender::Female, // Use Female as default in case of manipulation
        };

        set_gender(g);
    };

    view! {
        <select class="input input-bordered" on:change=input_changed>
            <option value="F">Female</option>
            <option value="M">Male</option>
        </select>
    }
}

#[component]
pub fn InputDistance(#[prop(into)] set_distance: WriteSignal<u32>) -> impl IntoView {
    let input_changed = move |ev| {
        let value = event_target_value(&ev);
        let dist = match value.as_str() {
            "25" => 25,
            "50" => 50,
            _ => 25,
        };

        set_distance(dist);
    };

    view! {
        <select class="input input-bordered" on:change=input_changed>
            <option value="25">25 Meters</option>
            <option value="50">50 Meters</option>
        </select>
    }
}

#[component]
pub fn InputDisqualified(#[prop(into)] set_disqualified: WriteSignal<bool>) -> impl IntoView {
    let input_changed = move |ev| {
        set_disqualified(event_target_checked(&ev));
    };

    view! {
        <input type="checkbox" class="checkbox" on:change=input_changed/>
    }
}

#[component]
pub fn InputStroke(#[prop(into)] set_stroke: WriteSignal<model::Stroke>) -> impl IntoView {
    let input_changed = move |ev| {
        let value = event_target_value(&ev);
        let s = match value.as_str() {
            "Butterfly" => model::Stroke::Butterfly,
            "Back" => model::Stroke::Back,
            "Breast" => model::Stroke::Breast,
            "Freestyle" => model::Stroke::Freestyle,
            _ => model::Stroke::Butterfly,
        };

        set_stroke(s);
    };

    view! {
        <select class="input input-bordered" on:change=input_changed>
            <option value="Butterfly">Butterfly</option>
            <option value="Back">Back</option>
            <option value="Breast">Breast</option>
            <option value="Freestyle">Freestyle</option>
        </select>
    }
}

#[component]
pub fn InputName(#[prop(into)] set_name: WriteSignal<String>) -> impl IntoView {
    let input_changed = move |ev| {
        let name = event_target_value(&ev);
        set_name(name);
    };

    view! {
        <input type="text" class="input input-bordered" on:change=input_changed required />
    }
}

#[component]
pub fn InputDate(#[prop(into)] set_date: WriteSignal<Option<chrono::NaiveDate>>) -> impl IntoView {
    let input_changed = move |ev| {
        let date = event_target_value(&ev);
        let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok();
        set_date(date);
    };

    view! {
        <input class="input input-bordered" type="date" on:change=input_changed required/>
    }
}

#[component]
pub fn InputGroup(#[prop(into)] set_group_id: WriteSignal<Option<Uuid>>) -> impl IntoView {
    let input_changed = move |ev| {
        let value = event_target_value(&ev);
        let group_id = Uuid::parse_str(&value).ok();
        set_group_id(group_id);
    };

    let available_groups = create_resource(
        || (),
        |_| async move { api_client::list_groups().await.unwrap() },
    );

    view! {
        <select class="input input-bordered" on:change=input_changed>
            <Transition>
                <For each=move || available_groups().unwrap_or_default() key=|g| g.id let:group>
                    <option value={group.id.to_string()}>{group.name}</option>
                </For>
            </Transition>
        </select>
    }
}

#[component]
pub fn CellDate(date: chrono::NaiveDate) -> impl IntoView {
    view! {
        <td><BirthdayDisplay birthday=date /></td>
    }
}

#[component]
pub fn HeadingsParticipant() -> impl IntoView {
    view! {
        <th>Code</th>
        <th>Last Name</th>
        <th>First Name</th>
        <th>Gender</th>
        <th>Birthday</th>
    }
}

#[component]
pub fn CellsParticipant(participant: model::Participant) -> impl IntoView {
    view! {
        <td>{participant.short_code}</td>
        <td>{participant.last_name}</td>
        <td>{participant.first_name}</td>
        <CellGender gender=participant.gender />
        <CellDate date=participant.birthday />
    }
}
