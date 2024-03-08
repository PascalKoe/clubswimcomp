use clubswimcomp_types::model;
use leptos::*;

use crate::components::values;
use crate::components::ActionType;

#[component]
pub fn Gender(#[prop(into)] gender: MaybeSignal<model::Gender>) -> impl IntoView {
    view! {
        <td>
            <values::Gender gender />
        </td>
    }
}

#[component]
pub fn Distance(#[prop(into)] distance: MaybeSignal<u32>) -> impl IntoView {
    view! {
        <td>
            <values::Distance distance />
        </td>
    }
}

#[component]
pub fn Stroke(#[prop(into)] stroke: MaybeSignal<model::Stroke>) -> impl IntoView {
    view! {
        <td>
            <values::Stroke stroke />
        </td>
    }
}

#[component]
pub fn Time(#[prop(into)] millis: MaybeSignal<u32>) -> impl IntoView {
    view! {
        <td>
            <values::Time millis />
        </td>
    }
}

#[component]
pub fn Disqualified(#[prop(into)] disqualified: MaybeSignal<bool>) -> impl IntoView {
    view! {
        <td>
            <values::Disqualified disqualified />
        </td>
    }
}

#[component]
pub fn Date(#[prop(into)] date: MaybeSignal<chrono::NaiveDate>) -> impl IntoView {
    view! {
        <td>
            <values::Date date />
        </td>
    }
}

#[component]
pub fn FinaPoints(#[prop(into)] fina_points: MaybeSignal<u32>) -> impl IntoView {
    view! {
        <td>
            <values::FinaPoints fina_points />
        </td>
    }
}

#[component]
pub fn ShortCode(#[prop(into)] short_code: MaybeSignal<String>) -> impl IntoView {
    view! {
        <td>
            <values::ShortCode short_code />
        </td>
    }
}

#[component]
pub fn Name(#[prop(into)] name: MaybeSignal<String>) -> impl IntoView {
    view! {
        <td>
            {name}
        </td>
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
) -> impl IntoView {
    view! {
        <td class="w-0">
            <super::Button action_type>
                {children()}
            </super::Button>
        </td>
    }
}

#[component]
pub fn Link(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
    #[prop(into)] href: String,
) -> impl IntoView {
    view! {
        <td class="w-0">
            <super::Link action_type href>
                {children()}
            </super::Link>
        </td>
    }
}
