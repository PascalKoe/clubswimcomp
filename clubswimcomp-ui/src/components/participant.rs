use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::{api_client, components::*};

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
    #[prop(into)] registrations: MaybeSignal<Vec<model::ParticipantRegistration>>,
    #[prop(into)] participant_id: MaybeSignal<Uuid>,
    #[prop(into, optional)] on_unregister: Option<Callback<()>>,
) -> impl IntoView {
    let unregister_action = create_action(|input: &(Uuid, Uuid)| {
        let participant_id = input.0;
        let registration_id = input.1;
        async move {
            api_client::unregister_from_competition(participant_id, registration_id)
                .await
                .unwrap();
        }
    });

    let on_unregistered_effect = create_memo(move |_| {
        if !unregister_action.pending().get() && unregister_action.value().get().is_some() {
            if let Some(on_unregister) = on_unregister {
                on_unregister(());
            }
        }
    });

    let rows = move || {
        registrations()
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
                            <button class="btn btn-xs btn-error" on:click=move |_| unregister_action.dispatch((participant_id(), r.id))>
                                Unregister
                            </button>
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
        {on_unregistered_effect}
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
    #[prop(optional, into)] on_registered: Option<Callback<()>>,
    #[prop(into)] participant_id: MaybeSignal<Uuid>,
    #[prop(into)] competitions: MaybeSignal<Vec<model::Competition>>,
) -> impl IntoView {
    let register_for_competition = create_action(|input: &(Uuid, Uuid)| {
        let participant_id = input.0;
        let competition_id = input.1;

        async move {
            api_client::register_for_competition(participant_id, competition_id)
                .await
                .unwrap();
        }
    });

    let on_registered_effect = create_memo(move |_| {
        if !register_for_competition.pending().get()
            && register_for_competition.value().get().is_some()
        {
            if let Some(on_registered) = on_registered {
                on_registered(());
            }
        }
    });

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
                            <button class="btn btn-xs btn-secondary" on:click=move |_| register_for_competition.dispatch((participant_id(), r.id))>
                                Register
                            </button>
                        </td>
                    </tr>
                }
            })
            .collect_view()
    };

    view! {
        {on_registered_effect}
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
