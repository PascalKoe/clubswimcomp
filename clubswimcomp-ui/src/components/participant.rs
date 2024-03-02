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
    #[prop(into, optional)] on_result_removed: Option<Callback<()>>,
    #[prop(into, optional)] on_result_added: Option<Callback<()>>,
) -> impl IntoView {
    let (error_msg, set_error_msg) = create_signal(None);

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

    let remove_result_action = create_action(|registration_id: &Uuid| {
        let registration_id = *registration_id;
        async move { api_client::remove_registration_result(registration_id).await }
    });

    let on_remove_result_effect = create_memo(move |_| {
        if remove_result_action.pending().get() {
            return;
        }

        let action_result = remove_result_action.value().get();
        match action_result {
            Some(Ok(_)) => {
                if let Some(on_result_removed) = on_result_removed {
                    on_result_removed(());
                }
            }
            Some(Err(err)) => set_error_msg(Some(err)),
            None => return, // Action is still running
        };
    });

    let result_dialog_id = create_rw_signal(None);

    let rows = move || {
        registrations()
            .into_iter()
            .map(|r| {
                let competition_link = format!("/competitions/{}", r.competition.id);
                let has_result = r.result.is_some();
                view! {
                    <tr>
                        <CellIconLink href=competition_link>
                            <phosphor_leptos::MagnifyingGlass/>
                        </CellIconLink>
                        <CellsCompetition competition=r.competition />

                        {
                            // Display either Trash or Timer button based on the
                            // existance of an registration result
                        }
                        <Show when=move || has_result>
                            <CellIconButton
                                action_type=ActionType::Error
                                on:click=move |_| remove_result_action.dispatch(r.id)
                            >
                                <phosphor_leptos::Trash/>
                            </CellIconButton>
                        </Show>
                        <Show when=move || !has_result>
                            <CellIconButton
                                action_type=ActionType::Secondary
                                on:click=move |_| result_dialog_id.set(Some(r.id))
                            >
                                <phosphor_leptos::Timer/>
                            </CellIconButton>
                        </Show>

                        <CellsResult result=r.result />

                        <CellIconButton
                            action_type=ActionType::Error
                            on:click=move |_| unregister_action.dispatch((participant_id(), r.id))
                        >
                            "Unregister"
                        </CellIconButton>
                    </tr>
                }
            })
            .collect_view()
    };

    view! {
        {on_unregistered_effect}
        {on_remove_result_effect}
        <AddResultDialog registration_id=result_dialog_id on_result_added=on_result_added />

        { move || error_msg().map(|err| view! { <p class="text-error">{err}</p>}) }
        <div class="overflow-x-auto">
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th></th>
                        <HeadingsCompetition />
                        <th></th>
                        <HeadingsResult />
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
pub fn AvailableCompetitionsTable(
    #[prop(into)] participant_id: Uuid,
    #[prop(default = None, into)] on_registered: Option<Callback<Uuid>>,
) -> impl IntoView {
    let (error_msg, set_error_msg) = create_signal(None);

    let available_competitions = create_resource(
        move || participant_id,
        |participant_id| {
            let participant_id = participant_id;
            async move {
                api_client::available_competitions_for_registration(participant_id)
                    .await
                    .unwrap()
            }
        },
    );

    #[derive(Clone)]
    struct RegisterAction {
        participant_id: Uuid,
        competition_id: Uuid,
    }
    let register_action = create_action(|input: &RegisterAction| {
        let input = input.clone();
        async move {
            api_client::register_for_competition(input.participant_id, input.competition_id).await
        }
    });

    let register_action_done = move || {
        let response = register_action.value()();
        match response {
            Some(Ok(registration_id)) => {
                if let Some(on_registered) = on_registered {
                    on_registered(registration_id);
                }
            }
            Some(Err(e)) => set_error_msg(Some(e)),
            None => return,
        };

        register_action.value().set(None);
    };

    let on_register = Callback::new(move |competition_id| {
        let input = RegisterAction {
            participant_id,
            competition_id,
        };
        register_action.dispatch(input);
    });

    view! {
        {register_action_done}
        <div class="overflow-x-auto">
            {move || error_msg().map(|e| view! {<p class="text-error font-bold">{e}</p>})}
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th></th>
                        <HeadingsCompetition />
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    <Transition>
                        <For each=move || available_competitions().unwrap_or_default() key=|c| c.id let:competition>
                            <AvailableCompetitionsRow competition on_register />
                        </For>
                    </Transition>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn AvailableCompetitionsRow(
    #[prop(into)] competition: model::Competition,
    #[prop(into)] on_register: Callback<Uuid>,
) -> impl IntoView {
    let competition_id = competition.id;

    view! {
        <tr>
            <CellIconLink href=format!("/competitions/{}", competition.id)>
                <phosphor_leptos::MagnifyingGlass />
            </CellIconLink>

            <CellsCompetition competition />

            <CellIconButton action_type=ActionType::Secondary on:click=move |_| on_register(competition_id)>
                Register
            </CellIconButton>
        </tr>
    }
}
