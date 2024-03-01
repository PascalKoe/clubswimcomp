use clubswimcomp_types::model;
use leptos::*;

mod competition;
mod page;
mod participant;

pub use competition::*;
pub use page::*;
pub use participant::*;

#[component]
pub fn GenderDisplay(gender: model::Gender, #[prop(optional)] short: bool) -> impl IntoView {
    move || match (gender, short) {
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
