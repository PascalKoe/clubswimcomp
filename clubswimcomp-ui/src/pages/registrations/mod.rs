use clubswimcomp_types::model;
use leptos::{ev::load, *};
use leptos_router::*;
use uuid::Uuid;

use crate::{api_client, components::*};

#[component]
pub fn ResultIngest() -> impl IntoView {
    let (error_msg, set_error_msg) = create_signal(None);
    let (registration_id, set_registration_id) = create_signal(None);

    let on_scanned = Callback::new(move |value: String| {
        let registration_id = Uuid::parse_str(&value).ok();
        set_registration_id(registration_id);
    });

    let load_registration_details = create_resource(registration_id, |registration_id| {
        let registration_id = registration_id.clone();
        async move {
            let Some(registration_id) = registration_id else {
                return None;
            };

            api_client::registration_details(registration_id).await.ok()
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
                load_registration_details.refetch();
            }
            Some(Err(err)) => set_error_msg(Some(err)),
            None => return, // Action is still running
        };
    });

    view! {
        <PageLayout>
            <PageTitle title="Scan Registration Cards" subtitle="Add a result to a registration by scanning the QR code on the registration card.".to_string().into() />
            <ActionRow>
                <Scanner value_scanned=on_scanned />
            </ActionRow>

            {move || error_msg().map(|e| view! {<p class="text-error font-bold">{e}</p>})}

            <Transition>
                {
                    move || load_registration_details().map(|rd| rd.map(|rd| view! {
                        {/* Participant and Competition info next to each other */}
                        <SectionTitle title="Registration Details"/>
                        <div class="flex flex-row w-full">
                            <div class="">
                                <data::ParticipantInfo participant=rd.participant.clone() />
                            </div>
                            <div class="flex-1">
                                <data::CompetitionInfo competition=rd.competition.clone() />
                            </div>
                        </div>

                        {/* The form to add the result to the registration */}
                        <SectionTitle title="Result for Registration"/>
                        {
                            if let Some(result) = rd.result {
                                view! {
                                    <ActionRow>
                                        <ActionButton action_type=ActionType::Error on:click=move |_| remove_result_action.dispatch(rd.id)>
                                            <phosphor_leptos::Trash />
                                            Delete
                                            {on_remove_result_effect}
                                        </ActionButton>
                                    </ActionRow>
                                    <ResultInfoTable registration_result=result />
                                }.into_view()
                            } else {
                                view! {
                                    <AddResultForm
                                        registration_id=rd.id
                                        on_added=move |_| load_registration_details.refetch()
                                        on_cancel=move |_| set_registration_id(None)
                                    />
                                }
                            }
                        }



                    }))}
            </Transition>
        </PageLayout>
    }
}
