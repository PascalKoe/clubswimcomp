use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

mod competition;
pub mod data;
mod group;
mod page;
mod participant;
mod registrations;
mod scanner;
pub mod tables;
pub mod values;

pub use competition::*;
pub use group::*;
pub use page::*;
pub use participant::*;
pub use registrations::*;
pub use scanner::*;
use uuid::Uuid;

use crate::api_client;

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
        <select class="input input-bordered" on:input=input_changed>
            <option selected></option>
            <Transition>
                <For each=move || available_groups().unwrap_or_default() key=|g| g.id let:group>
                    <option value={group.id.to_string()}>{group.name}</option>
                </For>
            </Transition>
        </select>
    }
}
