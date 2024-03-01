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
