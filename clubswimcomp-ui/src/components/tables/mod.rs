use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use crate::components::values;

use super::ActionType;

pub mod cells;
pub mod columns;

#[component]
pub fn Button(
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
pub fn Link(
    children: Children,
    #[prop(default = Default::default())] action_type: ActionType,
    #[prop(into)] href: String,
) -> impl IntoView {
    let btn_class = action_type.as_class();
    view! {
        <A href class=format!("btn btn-xs {btn_class}")>
            {children()}
        </A>
    }
}

#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="table table-xs">
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn ParticipantOverview(
    #[prop(into)] participants: MaybeSignal<Vec<model::Participant>>,
) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <columns::ParticipantHeadings />
                    <th></th>
                </tr>
            </thead>
            <tbody>
                <For each=participants key=|p| p.id let:participant>
                    <ParticipantRow participant />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn ParticipantRow(participant: model::Participant) -> impl IntoView {
    let participant_link = format!("/participants/{}", participant.id);
    view! {
        <tr>
            <columns::Participant participant />
            <cells::Link href=participant_link>
                Details
            </cells::Link>
        </tr>
    }
}

#[component]
pub fn CompetitionOverview(
    #[prop(into)] competitions: MaybeSignal<Vec<model::Competition>>,
) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <columns::CompetitionHeadings />
                    <th></th>
                </tr>
            </thead>
            <tbody>
                <For each=competitions key=|c| c.id let:competition>
                    <CompetitionRow competition />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn CompetitionRow(competition: model::Competition) -> impl IntoView {
    let competition_link = format!("/competitions/{}", competition.id);
    view! {
        <tr>
            <columns::Competition competition />
            <cells::Link href=competition_link>
                Details
            </cells::Link>
        </tr>
    }
}

#[component]
pub fn GroupOverview(#[prop(into)] groups: MaybeSignal<Vec<model::Group>>) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <columns::GroupHeadings />
                    <th></th>
                </tr>
            </thead>
            <tbody>
                <For each=groups key=|g| g.id let:group>
                    <GroupRow group />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn GroupRow(group: model::Group) -> impl IntoView {
    let group_link = format!("/groups/{}", group.id);
    view! {
        <tr>
            <columns::Group group />
            <cells::Link href=group_link>
                Details
            </cells::Link>
        </tr>
    }
}

#[component]
pub fn RegistrationDetails(
    #[prop(into)] registration_details: MaybeSignal<Vec<model::RegistrationDetails>>,
) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <th></th>
                    <columns::ParticipantHeadings />
                    <columns::CompetitionHeadings />
                </tr>
            </thead>
            <tbody>
                <For each=registration_details key=|r| r.id let:registration_details>
                    <RegistrationDetailsRow registration_details/>
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn RegistrationDetailsRow(registration_details: model::RegistrationDetails) -> impl IntoView {
    let participant_link = format!("/participants/{}", registration_details.participant.id);
    view! {
        <tr>
            <cells::Link href=participant_link>
                <phosphor_leptos::User />
            </cells::Link>
            <columns::Participant participant=registration_details.participant />
            <columns::Competition competition=registration_details.competition />
        </tr>
    }
}

#[component]
pub fn GroupScores(
    #[prop(into)] group_scores: MaybeSignal<Vec<model::GroupScore>>,
) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <th></th>
                    <columns::ParticipantHeadings />
                    <columns::GroupScoreHeadings />
                </tr>
            </thead>
            <tbody>
                <For each=group_scores key=|g| g.participant.id let:group_score>
                    <GroupScoreRow group_score />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn GroupScoreRow(group_score: model::GroupScore) -> impl IntoView {
    let participant_link = format!("/participants/{}", group_score.participant.id);
    view! {
        <tr>
            <cells::Link href=participant_link>
                <phosphor_leptos::User />
            </cells::Link>
            <columns::Participant participant=group_score.participant.clone() />
            <columns::GroupScore group_score=group_score/>
        </tr>
    }
}

#[component]
pub fn CompetitionScores(scores: Vec<model::CompetitionScore>) -> impl IntoView {
    let (mut top_scores, mut scores): (Vec<_>, Vec<_>) =
        scores.into_iter().partition(|s| s.rank <= 3);
    top_scores.sort_by_key(|s| s.rank);
    scores.sort_by(|x, y| x.participant.last_name.cmp(&y.participant.last_name));

    view! {
        <Table>
            <thead>
                <tr>
                    <th>Last Name</th>
                    <th>First Name</th>
                    <th>Age</th>
                    <th>FINA Points</th>
                    <th>Time</th>
                    <th>Rank</th>
                </tr>
            </thead>
            <tbody>
                <For each=move || top_scores.clone() key=|s| s.participant.id let:cs>
                    <CompetitionScoresRow competition_score=cs show_rank=true />
                </For>

                <tr class="h-8">
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>

                <For each=move || scores.clone() key=|s| s.participant.id let:cs>
                    <CompetitionScoresRow competition_score=cs show_rank=false />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn CompetitionScoresRow(
    competition_score: model::CompetitionScore,
    show_rank: bool,
) -> impl IntoView {
    view! {
        <tr>
            <td>{competition_score.participant.last_name}</td>
            <td>{competition_score.participant.first_name}</td>
            <td>{competition_score.participant.age}</td>
            <td><values::FinaPoints fina_points=competition_score.fina_points /></td>
            <td><values::Time millis=competition_score.time /></td>
            <td>
                {
                    if show_rank {
                        competition_score.rank.to_string()
                    } else {
                        "".to_string()
                    }
                }
            </td>
        </tr>
    }
}

#[component]
pub fn CompetitionDisqualifications(
    mut disqualifications: Vec<model::CompetitionRegistration>,
) -> impl IntoView {
    disqualifications.sort_by(|x, y| x.participant.last_name.cmp(&y.participant.last_name));

    view! {
        <Table>
            <thead>
                <tr>
                    <th>Last Name</th>
                    <th>First Name</th>
                    <th>Age</th>
                </tr>
            </thead>
            <tbody>
                <For each=move || disqualifications.clone() key=|d| d.participant.id let:d>
                    <CompetitionDisqualificationsRow disqualification=d />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn CompetitionDisqualificationsRow(
    disqualification: model::CompetitionRegistration,
) -> impl IntoView {
    view! {
        <tr>
            <td>{disqualification.participant.last_name}</td>
            <td>{disqualification.participant.first_name}</td>
            <td>{disqualification.participant.age}</td>
        </tr>
    }
}

#[component]
pub fn CompetitionMissingResults(
    mut missing: Vec<model::CompetitionRegistration>,
) -> impl IntoView {
    missing.sort_by(|x, y| x.participant.last_name.cmp(&y.participant.last_name));

    view! {
        <Table>
            <thead>
                <tr>
                    <th>Last Name</th>
                    <th>First Name</th>
                    <th>Age</th>
                </tr>
            </thead>
            <tbody>
                <For each=move || missing.clone() key=|m| m.participant.id let:m>
                    <CompetitionMissingResultsRow missing=m />
                </For>
            </tbody>
        </Table>
    }
}

#[component]
pub fn CompetitionMissingResultsRow(missing: model::CompetitionRegistration) -> impl IntoView {
    view! {
        <tr>
            <td>{missing.participant.last_name}</td>
            <td>{missing.participant.first_name}</td>
            <td>{missing.participant.age}</td>
        </tr>
    }
}
