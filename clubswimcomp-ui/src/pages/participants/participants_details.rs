use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::api_client;
use crate::components::*;

#[component]
pub fn ParticipantDetails() -> impl IntoView {
    let refresh_trigger = create_trigger();

    let params = use_params_map();
    let participant_id = move || {
        params()
            .get("participant_id")
            .map(|s| Uuid::parse_str(s).ok())
            .flatten()
            .unwrap()
    };

    let participant_details = create_local_resource(participant_id, move |p| async move {
        api_client::participant_details(p).await.unwrap()
    });

    let available_competitions = create_local_resource(participant_id, move |p| async move {
        api_client::available_competitions_for_registration(p)
            .await
            .unwrap()
    });

    let refetch_data = move |_| {
        participant_details.refetch();
        available_competitions.refetch();
    };

    view! {
        <PageLayout>
            <PageTitle
                title="Participant Details"
                subtitle="Details about a specific participant all of hist registrations.".to_string().into()
            />
            <Suspense fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    // FIXME: Participant Start Card Link

                    move || participant_details.get().map(|pd| {
                        let start_card_link = format!("http://localhost:3000/participants/{}/registrations/cards", pd.participant.id);
                        view! {
                            <div class="mb-8">
                                <A target="about:blank" href=start_card_link class="btn btn-sm btn-primary rounded-full">
                                    <phosphor_leptos::Printer />
                                    Print Registration Cards
                                </A>
                            </div>

                            <ParticipantInfoTable participant=pd.participant.clone() />

                            <SectionTitle
                                title="Registrations".to_string()
                                subtitle="This is an overview over all of the registrations that exist for this participant.".to_string().into()
                            />
                            <ParticipantRegistrationsTable
                                participant_id=pd.participant.id
                                registrations=pd.registrations
                                on_unregister=refetch_data
                            />

                            <SectionTitle
                                title="Available Competitions".to_string()
                                subtitle="This is an list of competitions that can be joined by the participant.".to_string().into()
                            />
                            <ParticipantAvailableCompetitionsTable
                                on_registered=refetch_data
                                participant_id=pd.participant.id
                                competitions=available_competitions.get().unwrap_or_default()
                            />
                        }
                    })
                }
            </Suspense>
        </PageLayout>
    }
}
