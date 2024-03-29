use leptos::*;
use leptos_router::*;

use crate::{api_client, components::*};

#[component]
pub fn ParticipantOverview() -> impl IntoView {
    let participants = create_local_resource(
        || (),
        move |_| async { api_client::list_participants().await.unwrap() },
    );

    let add_participant_dialog_open = create_rw_signal(false);
    let participant_added = Callback::new(move |_| participants.refetch());
    let add_participant_clicked = move |_| {
        add_participant_dialog_open.set(true);
    };

    view! {
        <PageLayout>
            <PageTitle
                title="Participants Overview"
                subtitle="An overview about every single participant that exists for this event.".to_string().into()
            />
            <AddParticipantDialog on_participant_added=participant_added show=add_participant_dialog_open/>
            <ActionRow>
                <button class="btn btn-sm btn-primary rounded-full" on:click=add_participant_clicked>
                    <phosphor_leptos::Plus />
                    Add Participant
                </button>
            </ActionRow>

            <SectionTitle title="Participants" subtitle="List of all participants.".to_string().into() />
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || participants.get().map(|p|
                        view! {
                            <tables::ParticipantOverview participants=p />
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
