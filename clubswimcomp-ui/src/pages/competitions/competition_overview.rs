use leptos::*;
use leptos_router::*;

use crate::api_client;
use crate::components::*;

#[component]
pub fn CompetitionOverview() -> impl IntoView {
    let competitions = create_local_resource(
        || (),
        move |_| async move { api_client::list_competitions().await.unwrap() },
    );

    view! {
        <PageLayout>
            <PageTitle
                title="Competition Overview"
                subtitle="An overview about every single competition that exists for this event.".to_string().into()
            />
            <Suspense fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || competitions.get().map(|c|
                        view! {
                            <div class="mb-8">
                                <A href="/competitions/add" class="btn btn-sm btn-primary rounded-full">
                                    <phosphor_leptos::Plus />
                                    Add a new competition
                                </A>
                            </div>
                            <SectionTitle title="Competitions" subtitle="List of all competitions.".to_string().into() />
                            <CompetitionOverviewTable competitions=c />
                        }
                    )
                }
            </Suspense>
        </PageLayout>
    }
}
