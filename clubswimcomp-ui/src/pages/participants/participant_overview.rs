use leptos::*;
use leptos_router::*;

use crate::{api_client, components::*};

#[component]
pub fn ParticipantOverview() -> impl IntoView {
    let participants = create_local_resource(
        || (),
        move |_| async { api_client::participants_overview().await.unwrap() },
    );

    view! {
        <PageLayout>
            <PageTitle
                title="Participants Overview"
                subtitle="An overview about every single participant that exists for this event.".to_string().into()
            />
            <Suspense fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || participants.get().map(|p|
                        view! {
                            <div class="mb-8">
                                <A href="/competitions/add" class="btn btn-sm btn-primary rounded-full">
                                    <phosphor_leptos::Plus />
                                    Add Participant
                                </A>
                            </div>
                            <SectionTitle title="Participants" subtitle="List of all participants.".to_string().into() />
                            <ParticipantOverviewTable participants=p />
                        }
                    )
                }
            </Suspense>
        </PageLayout>
    }
}
