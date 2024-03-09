use leptos::*;
use participant::tables::{cells, columns};

use crate::components::*;

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
                        <cells::Link href=competition_link>
                            <phosphor_leptos::MagnifyingGlass/>
                        </cells::Link>
                        <columns::Competition competition=r.competition />

                        {
                            // Display either Trash or Timer button based on the
                            // existance of an registration result
                        }
                        <Show when=move || has_result>
                            <cells::Button
                                action_type=ActionType::Error
                                on:click=move |_| remove_result_action.dispatch(r.id)
                            >
                                <phosphor_leptos::Trash/>
                            </cells::Button>
                        </Show>
                        <Show when=move || !has_result>
                            <cells::Button
                                action_type=ActionType::Secondary
                                on:click=move |_| result_dialog_id.set(Some(r.id))
                            >
                                <phosphor_leptos::Timer/>
                            </cells::Button>
                        </Show>

                        <columns::RegistrationResult registration_result=r.result />

                        <cells::Button
                            action_type=ActionType::Error
                            on:click=move |_| unregister_action.dispatch((participant_id(), r.id))
                        >
                            "Unregister"
                        </cells::Button>
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
                        <columns::CompetitionHeadings />
                        <th></th>
                        <columns::RegistrationResultHeadings />
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
                        <columns::CompetitionHeadings />
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
            <cells::Link href=format!("/competitions/{}", competition.id)>
                <phosphor_leptos::MagnifyingGlass />
            </cells::Link>

            <columns::Competition competition />

            <cells::Button action_type=ActionType::Secondary on:click=move |_| on_register(competition_id)>
                Register
            </cells::Button>
        </tr>
    }
}

#[component]
pub fn AddParticipantForm(
    on_participant_added: Callback<Uuid>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (error_message, set_error_message) = create_signal(None);

    let (first_name, set_first_name) = create_signal(String::new());
    let (last_name, set_last_name) = create_signal(String::new());
    let (gender, set_gender) = create_signal(model::Gender::Female);
    let (birthday, set_birthday) = create_signal(None);
    let (group_id, set_group_id) = create_signal(None);

    #[derive(Clone)]
    struct AddParticipantAction {
        first_name: String,
        last_name: String,
        gender: model::Gender,
        birthday: chrono::NaiveDate,
        group_id: Uuid,
    }
    let add_participant_action = create_action(|input: &AddParticipantAction| {
        let input = input.clone();
        async move {
            api_client::add_participant(
                input.first_name,
                input.last_name,
                input.gender,
                input.birthday,
                input.group_id,
            )
            .await
        }
    });

    let on_participant_added_handler = move || match add_participant_action.value().get() {
        Some(Ok(participant_id)) => on_participant_added(participant_id),
        Some(Err(e)) => set_error_message(Some(e)),
        None => (),
    };

    let participant_saving = move || add_participant_action.pending().get();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let Some(birthday) = birthday() else {
            leptos::logging::warn!("Missing birthday");
            return;
        };
        let Some(group_id) = group_id() else {
            leptos::logging::warn!("Missing group id");
            return;
        };

        let input = AddParticipantAction {
            first_name: first_name(),
            last_name: last_name(),
            gender: gender(),
            birthday,
            group_id,
        };
        add_participant_action.dispatch(input);
    };

    let on_cancel_button_clicked = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        on_cancel(());
    };

    view! {
        {on_participant_added_handler}

        <form on:submit=on_submit>
            <FormItem label="Last Name">
                <InputName set_name=set_last_name />
            </FormItem>

            <FormItem label="First Name">
                <InputName set_name=set_first_name />
            </FormItem>

            <FormItem label="Gender">
                <InputGender set_gender />
            </FormItem>

            <FormItem label="Birthday">
                <InputDate set_date=set_birthday />
            </FormItem>

            <FormItem label="Group">
                <InputGroup set_group_id />
            </FormItem>
            {
                move|| error_message().map(|e| view!{<p class="text text-error">{e}</p>})
            }

            <div class="form-control w-full max-w-2xl mt-4">
                <input class="btn btn-primary" type="submit" value="Add Participant" disabled=participant_saving />
            </div>
            <div class="form-control w-full max-w-2xl mt-4">
                <button class="btn btn-neutral" on:click=on_cancel_button_clicked>
                    Cancel
                </button>
            </div>
        </form>
    }
}

#[component]
pub fn AddParticipantDialog(
    #[prop(into)] show: RwSignal<bool>,
    on_participant_added: Callback<Uuid>,
) -> impl IntoView {
    let on_added_callback = Callback::new(move |competition_id| {
        show.set(false);
        on_participant_added(competition_id);
    });

    let on_cancel = Callback::new(move |()| show.set(false));

    view! {
        <dialog class="modal bg-black bg-opacity-30" autofocus open=show>
            <div class="modal-box">
                <h3 class="text-xl text-black">Add a Participant</h3>
                <AddParticipantForm
                    on_participant_added=on_added_callback
                    on_cancel
                />
            </div>
        </dialog>
    }
}
