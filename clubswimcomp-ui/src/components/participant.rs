use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use crate::components::*;

#[component]
pub fn ParticipantInfoTable(
    #[prop(into)] participant: MaybeSignal<model::Participant>,
) -> impl IntoView {
    view! {
        <table class="table table-xs w-80">
            <tbody>
                <tr>
                    <td class="font-bold w-40">Code</td>
                    <td>{participant().short_code}</td>
                </tr>
                <tr>
                    <td class="font-bold w-40">Last Name</td>
                    <td>{participant().last_name}</td>
                </tr>
                <tr>
                    <td class="font-bold w-40">First Name</td>
                    <td>{participant().first_name}</td>
                </tr>
                <tr>
                    <td class="font-bold w-40">Gender</td>
                    <td><GenderDisplay gender={participant().gender}/></td>
                </tr>
                <tr>
                    <td class="font-bold w-40">Birthday</td>
                    <td><BirthdayDisplay birthday=participant().birthday /></td>
                </tr>

                <tr>
                    <td class="font-bold w-40">Age</td>
                    <td>{participant().age}</td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
pub fn ParticipantOverviewTable(
    #[prop(into)] participants: MaybeSignal<Vec<model::Participant>>,
) -> impl IntoView {
    let rows = move || {
        participants()
            .into_iter()
            .map(|p| {
                let details_link = format!("/participants/{}", p.id);
                view! {
                    <tr>
                        <td>{p.short_code}</td>
                        <td>{p.last_name}</td>
                        <td>{p.first_name}</td>
                        <td><GenderDisplay gender=p.gender /></td>
                        <td><BirthdayDisplay birthday=p.birthday /></td>
                        <td>{p.age}</td>
                        <td class="w-0">
                            <A class="btn btn-xs" href=details_link>Details</A>
                        </td>
                    </tr>
                }
            })
            .collect_view()
    };

    view! {
        <div class="overflow-x-auto">
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th>Code</th>
                        <th>Last Name</th>
                        <th>First Name</th>
                        <th>Gender</th>
                        <th>Birthday</th>
                        <th>Age</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ParticipantRegistrationsTable(
    #[prop(into)] registraions: MaybeSignal<Vec<model::ParticipantRegistration>>,
) -> impl IntoView {
    let rows = move || {
        registraions()
            .into_iter()
            .map(|r| {
                let competition_link = format!("/competitions/{}", r.competition.id);
                view! {
                    <tr>
                        <td class="w-0">
                            <A class="btn btn-xs" href=competition_link>
                                <phosphor_leptos::MagnifyingGlass />
                            </A>
                        </td>
                        <td><DistanceDisplay distance=r.competition.distance /></td>
                        <td><GenderDisplay gender=r.competition.gender /></td>
                        <td><StrokeDisplay stroke=r.competition.stroke/></td>
                        <td>
                            {
                                r.result.as_ref().map(|r| if r.disqualified {
                                    "Disqualified"
                                } else {
                                    "-"
                                })
                            }
                        </td>
                        <td>
                            {
                                r.result
                                    .as_ref()
                                    .map(|r| view! {
                                        <TimeDisplay millis=r.time_millis />
                                    })
                            }
                        </td>
                        <td class="w-0">
                            <A class="btn btn-xs btn-error" href="">Unregister</A>
                        </td>
                        <td class="w-0">
                            <A class="btn btn-xs btn-secondary" href="">Result</A>
                        </td>
                    </tr>
                }
            })
            .collect_view()
    };

    view! {
        <div class="overflow-x-auto">
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th></th>
                        <th>Distance</th>
                        <th>Gender</th>
                        <th>Stroke</th>
                        <th>Disqualified</th>
                        <th>Time</th>
                        <th></th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ParticipantAvailableCompetitionsTable(
    #[prop(into)] competitions: MaybeSignal<Vec<model::Competition>>,
) -> impl IntoView {
    let rows = move || {
        competitions()
            .into_iter()
            .map(|r| {
                let competition_link = format!("/competitions/{}", r.id);
                view! {
                    <tr>
                        <td class="w-0">
                            <A class="btn btn-xs" href=competition_link>
                                <phosphor_leptos::MagnifyingGlass />
                            </A>
                        </td>
                        <td><DistanceDisplay distance=r.distance /></td>
                        <td><GenderDisplay gender=r.gender /></td>
                        <td><StrokeDisplay stroke=r.stroke/></td>
                        <td class="w-0">
                            <A class="btn btn-xs btn-secondary" href="">Register</A>
                        </td>
                    </tr>
                }
            })
            .collect_view()
    };

    view! {
        <div class="overflow-x-auto">
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th></th>
                        <th>Distance</th>
                        <th>Gender</th>
                        <th>Stroke</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }
}
