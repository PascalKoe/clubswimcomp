use clubswimcomp_types::model;
use leptos::*;

#[component]
pub fn Gender(
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
pub fn Distance(#[prop(into)] distance: MaybeSignal<u32>) -> impl IntoView {
    move || format!("{} m", distance())
}

#[component]
pub fn Stroke(#[prop(into)] stroke: MaybeSignal<model::Stroke>) -> impl IntoView {
    move || match stroke() {
        model::Stroke::Butterfly => "Butterfly",
        model::Stroke::Back => "Backstroke",
        model::Stroke::Breast => "Breaststroke",
        model::Stroke::Freestyle => "Freestyle",
    }
}

#[component]
pub fn Date(#[prop(into)] date: MaybeSignal<chrono::NaiveDate>) -> impl IntoView {
    move || date().format("%Y-%m-%d").to_string()
}

#[component]
pub fn Time(#[prop(into)] millis: MaybeSignal<u32>) -> impl IntoView {
    let minutes = move || millis() / (60 * 1000);
    let seconds = move || (millis() / 1000) % 60;
    let hundreths = move || (millis() / 10) % 100;

    move || format!("{:02}:{:02},{:02}", minutes(), seconds(), hundreths())
}

#[component]
pub fn Disqualified(#[prop(into)] disqualified: MaybeSignal<bool>) -> impl IntoView {
    move || match disqualified() {
        true => "Disqualified",
        false => "-",
    }
}

#[component]
pub fn FinaPoints(#[prop(into)] fina_points: MaybeSignal<u32>) -> impl IntoView {
    move || format!("{} pt", fina_points())
}

#[component]
pub fn ShortCode(#[prop(into)] short_code: MaybeSignal<String>) -> impl IntoView {
    short_code
}
