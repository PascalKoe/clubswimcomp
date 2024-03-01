use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::api_client;
use crate::components::*;

#[component]
pub fn CompetitionDetails() -> impl IntoView {
    let params = use_params_map();
    let competition_id = move || {
        params()
            .get("competition_id")
            .map(|s| Uuid::parse_str(s).ok())
            .flatten()
            .unwrap()
    };

    let competition_details = create_local_resource(competition_id, move |c| async move {
        api_client::competition_details(c).await.unwrap()
    });

    view! {
        <PageLayout>
            <PageTitle
                title="Competition Details"
                subtitle="Details about a specific competition with all of it's registrations.".to_string().into()
            />
            <Suspense fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || competition_details.get().map(|cd|
                        view! {
                        <CompetitionInfoTable competition=cd.competition />

                        <SectionTitle
                            title="Registrations".to_string()
                            subtitle="This is an overview over all of the registrations that exist for this competition.".to_string().into()
                        />
                        <CompetitionRegistrationsTable registrations=cd.registrations />
                    })
                }
            </Suspense>
        </PageLayout>
    }
}
