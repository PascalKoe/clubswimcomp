use clubswimcomp_types::model;
use leptos::*;

use super::cells;

#[component]
pub fn Competition(#[prop(into)] competition: MaybeSignal<model::Competition>) -> impl IntoView {
    move || {
        let competition = competition();
        view! {
            <cells::Distance distance=competition.distance />
            <cells::Stroke stroke=competition.stroke />
            <cells::Gender gender=competition.gender />
            <cells::Time millis=competition.target_time />
        }
    }
}

#[component]
pub fn CompetitionHeadings() -> impl IntoView {
    view! {
        <th>Distance</th>
        <th>Stroke</th>
        <th>Gender</th>
        <th>Target Time</th>
    }
}

#[component]
pub fn RegistrationResultHeadings() -> impl IntoView {
    view! {
        <th>Disqualified</th>
        <th>Time</th>
        <th>FINA Points</th>
    }
}

#[component]
pub fn RegistrationResult(
    #[prop(into)] registration_result: MaybeSignal<Option<model::RegistrationResult>>,
) -> impl IntoView {
    move || {
        if let Some(registration_result) = registration_result() {
            view! {
                <cells::Disqualified disqualified=registration_result.disqualified />
                <cells::Time millis=registration_result.time_millis />
                <cells::FinaPoints fina_points=registration_result.fina_points />
            }
            .into_view()
        } else {
            view! {
                <td></td>
                <td></td>
                <td></td>
            }
            .into_view()
        }
    }
}

#[component]
pub fn ParticipantHeadings() -> impl IntoView {
    view! {
        <th>Code</th>
        <th>Last Name</th>
        <th>First Name</th>
        <th>Gender</th>
        <th>Birthday</th>
    }
}

#[component]
pub fn Participant(#[prop(into)] participant: MaybeSignal<model::Participant>) -> impl IntoView {
    move || {
        let participant = participant();
        view! {
            <cells::ShortCode short_code=participant.short_code />
            <cells::Name name=participant.last_name />
            <cells::Name name=participant.first_name />
            <cells::Gender gender=participant.gender />
            <cells::Date date=participant.birthday />
        }
    }
}

#[component]
pub fn GroupHeadings() -> impl IntoView {
    view! {
        <th>Group Name</th>
    }
}

#[component]
pub fn Group(#[prop(into)] group: MaybeSignal<model::Group>) -> impl IntoView {
    move || {
        let group = group();
        view! {
            <cells::Name name=group.name/>
        }
    }
}

#[component]
pub fn GroupScoreHeadings() -> impl IntoView {
    view! {
        <th>FINA Points</th>
        <th>Rank</th>
    }
}

#[component]
pub fn GroupScore(#[prop(into)] group_score: MaybeSignal<model::GroupScore>) -> impl IntoView {
    move || {
        let group_score = group_score();
        view! {
            <cells::FinaPoints fina_points=group_score.fina_points/>
            <cells::Name name=group_score.rank.to_string() />
        }
    }
}
